use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, LitStr};

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

#[proc_macro_attribute]
pub fn from_regex(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // DeriveInput is not 100% correct, but works as long as I only use the macro on structs and enums
    let derive_input = parse_macro_input!(input as DeriveInput);
    
    let data_enum = if let Data::Enum(data_enum) = &derive_input.data {
        data_enum
    } else {
        return syn::Error::new(
            derive_input.ident.span(),
            "Only enums for now"
        ).to_compile_error().into()
    };
    
    // create lazy lock regex statics
    let static_regexes = data_enum.variants
        .iter()
        .map(|variant|{
            let ident = format_ident!(
                "{}_{}_REGEX", derive_input.ident.to_string().to_uppercase(),
                variant.ident.to_string().to_uppercase()
            );
            let regex = get_regex_from_attributes(&variant.attrs).expect("Failed to get regex literal from attributes");
            quote! {static #ident: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(#regex).unwrap());}
        });
    
    let enum_ident = &derive_input.ident;
    
    // create the trait implementation
    let calls = data_enum.variants
        .iter()
        .map(|variant| {
            let static_ident = format_ident!(
                "{}_{}_REGEX", derive_input.ident.to_string().to_uppercase(),
                variant.ident.to_string().to_uppercase()
            );
            
            match &variant.fields {
                Fields::Named(f) => {
                    let ident = &variant.ident;
                    let params = f.named
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let ident = f.ident.as_ref().unwrap();
                            let ty = &f.ty;
                            quote! { #ident: capture.get(#i).unwrap().as_str().parse::<#ty>().unwrap() }
                        });
                    quote! {
                        if let Some(capture) = #static_ident.captures(haystack) {
                            return #enum_ident::#ident {
                                #(#params),*
                            }
                        }
                    }
                }
                Fields::Unnamed(f) => {
                    let ident = &variant.ident;
                    let params = f.unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let i = i + 1;
                            let ty = &f.ty;
                            quote! { capture.get(#i).unwrap().as_str().parse::<#ty>().unwrap() }
                        });
                    
                    quote! {
                        if let Some(capture) = #static_ident.captures(haystack) {
                            return #enum_ident::#ident(#(#params),*)
                        }
                    }
                }
                Fields::Unit => {
                    let ident = &variant.ident;
                    quote! {
                        if #static_ident.is_match(haystack) {
                            return #enum_ident::#ident;
                        }
                    }
                }
            }
        });
   
    // remove the custom helper attributes from the original input
    let mut cleaned_input = derive_input.clone();
    if let Data::Enum(data_enum) = &mut cleaned_input.data {
        data_enum.variants
            .iter_mut()
            .for_each(|variant| variant.attrs = variant.attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("reg"))
                .cloned()
                .collect());
    }

    quote!{
        #(#static_regexes)*

        #cleaned_input
        
        impl #enum_ident {
            fn from_regex(haystack: &str) -> Self {
                #(#calls)*

                panic!("ahh!")
            }
        }
    }.into()
}

fn get_regex_from_attributes<'a>(attrs: impl IntoIterator<Item=&'a Attribute>) -> Option<LitStr> {
    let regex_attribute = attrs
        .into_iter()
        .find(|attr| {
            match attr.path().segments.last() {
                Some(path) => path.ident.to_string() == "reg".to_string(),
                None => false
            }
        })?;
    
    regex_attribute.parse_args::<LitStr>().ok()
}