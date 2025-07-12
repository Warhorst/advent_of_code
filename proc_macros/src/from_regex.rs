use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Fields, FieldsNamed, FieldsUnnamed, Item, ItemEnum, LitStr, Variant};
// the plan:
// #[derive(FromRegex)]
// enum MyVariants {
//      #[reg(r#".*foo.*"#)] no values, only match
//      Foo,
//      #[reg(r#"bar(\d): ([a-z]+)"#)] has values, so use captures in the order of the fields
//      Bar(usize, String)
//      #[reg(r#"(\d):baz"#)]
//      Baz { some_value: usize }
// }

// creates something like this:
// // first, lazy lock regex initializations
// static MYVARIANTS_FOO_REGEX = ...
// ... same for the other variants
//
// from_regex(haystack: &str) -> Self {
//      if MYVARIANTS_FOO_REGEX.is_match(haystack) {
//          return Foo;
//      }
//      
//      if let Some(capture) = MYVARIANTS_BAR_REGEX.captures(haystack) {
//          return Bar(capture.get(1).unwrap().as_str().parse::<usize>().unwrap(), ...)
//      }
// 
//      if let Some(capture) = MYVARIANTS_BAZ_REGEX.captures(haystack) {
//          return Baz { some_value: ... }
//      }
//      
//      panic!("Could not parse from input '{haystack}'")
// }

// todo refactoring (for example don't panic, but create compiler errors on failure)
//  and implement this for structs too

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let item_enum = match item {
        Item::Enum(item_enum) => item_enum,
        _ => return syn::Error::new(
                item.span(),
                "Only enums for now"
            ).to_compile_error().into()
    };

    let static_regexes = match create_static_regexes_from_variants(&item_enum) {
        Ok(sr) => sr,
        Err(err) => return err
    };

    let from_regex_implementation = create_from_regex_implementation(&item_enum);

    // remove the custom helper attributes from the original input
    let mut cleaned_enum = item_enum.clone();
    cleaned_enum.variants
        .iter_mut()
        .for_each(|variant| variant.attrs = variant.attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("reg"))
            .cloned()
            .collect()
        );

    quote!{
        #cleaned_enum

        #static_regexes

        #from_regex_implementation
    }.into()
}

fn create_static_regexes_from_variants(item_enum: &ItemEnum) -> Result<proc_macro2::TokenStream, TokenStream> {
    let regexes = item_enum.variants
        .iter()
        .map(|variant| {
            let ident = format_ident!(
                "{}_{}_REGEX", item_enum.ident.to_string().to_uppercase(),
                variant.ident.to_string().to_uppercase()
            );

            match get_regex_from_attributes(&variant.attrs) {
                Some(regex) => Ok(quote! {static #ident: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(#regex).unwrap());}),
                None => Err(syn::Error::new(
                    variant.span(),
                    format!("The variant {} does not have the reg attribute defined!", variant.ident),
                ).to_compile_error().into())
            }
        })
        .collect::<Result<Vec<_>, TokenStream>>()?;

    Ok(quote! {#(#regexes)*})
}

fn get_regex_from_attributes<'a>(attrs: impl IntoIterator<Item=&'a Attribute>) -> Option<LitStr> {
    let regex_attribute = attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("reg"))?;

    regex_attribute.parse_args::<LitStr>().ok()
}

fn create_from_regex_implementation(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let enum_ident = &item_enum.ident;
    let generics = &item_enum.generics;
    let where_clause = &generics.where_clause;

    let ifs = item_enum.variants
        .iter()
        .map(|variant| {
            let static_ident = format_ident!(
                "{}_{}_REGEX", item_enum.ident.to_string().to_uppercase(),
                variant.ident.to_string().to_uppercase()
            );

            match &variant.fields {
                Fields::Named(f) => create_named_variant_if(enum_ident, &static_ident, variant, f),
                Fields::Unnamed(f) => create_unnamed_variant_if(enum_ident, &static_ident, variant, f),
                Fields::Unit => create_unit_variant_if(enum_ident, &static_ident, variant)
            }
        });

    quote! {
        impl #generics #enum_ident #generics #where_clause {
            fn from_regex(haystack: &str) -> Self {
                #(#ifs)*

                panic!("None of the variant regexes matches the haystack '{haystack}!'")
            }
        }
    }
}

fn create_named_variant_if(
    enum_ident: &Ident,
    static_ident: &Ident,
    variant: &Variant,
    f: &FieldsNamed
)  -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let params = f.named
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let i = i + 1;
            let ident = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            quote! { #ident: capture
                .get(#i)
                .expect(&format!("Expected {} capture in the haystack", #i))
                .as_str()
                .parse::<#ty>()
                .expect(&format!("Failed to parse '{}' as {}", capture.get(#i).unwrap().as_str(), stringify!(#ty)))
            }
        });

    quote! {
        if let Some(capture) = #static_ident.captures(haystack) {
            return #enum_ident::#ident {
                #(#params),*
            }
        }
    }
}

fn create_unnamed_variant_if(
    enum_ident: &Ident,
    static_ident: &Ident,
    variant: &Variant,
    f: &FieldsUnnamed
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let params = f.unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let i = i + 1;
            let ty = &f.ty;
            quote! { capture
                .get(#i)
                .expect(&format!("Expected {} capture in the haystack", #i))
                .as_str()
                .parse::<#ty>()
                .expect(&format!("Failed to parse '{}' as {}", capture.get(#i).unwrap().as_str(), stringify!(#ty)))
            }
        });

    quote! {
        if let Some(capture) = #static_ident.captures(haystack) {
            return #enum_ident::#ident(#(#params),*)
        }
    }
}

fn create_unit_variant_if(
    enum_ident: &Ident,
    static_ident: &Ident,
    variant: &Variant
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    quote! {
        if #static_ident.is_match(haystack) {
            return #enum_ident::#ident;
        }
    }
}
