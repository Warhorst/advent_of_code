use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, LitChar, Meta, Token};
use syn::punctuated::Punctuated;

pub (crate) fn create(item: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(item as ItemEnum);
    let ident = &item_enum.ident;

    let variants_with_chars = item_enum.variants
        .iter()
        .map(|variant| {
            let tile_attribute = variant.attrs
                .iter()
                .find(|attr| attr
                    .path()
                    .get_ident()
                    .map(|ident| ident.to_string())
                    .filter(|name| name.as_str() == "t")
                    .is_some()
                )
                .expect("Every enum attribute must have the 'tile' attribute");

            if let Meta::List(list) = &tile_attribute.meta {
                (variant, list
                    .parse_args_with(Punctuated::<LitChar, Token![,]>::parse_terminated)
                    .expect("Failed to parse arguments to char")
                    .iter()
                    .next()
                    .expect("Failed to retrieve the char argument").clone())
            } else {
                panic!("Every enum variant needs a char as parameter")
            }
        }).collect::<Vec<_>>();

    // remove the attributes from the variants, as they are no longer valid tokens
    let mut item_enum = item_enum.clone();
    item_enum.variants.iter_mut().for_each(|var| var.attrs.clear());

    let from_char_match_arms = variants_with_chars
        .iter()
        .map(|(variant, char)| (&variant.ident, char))
        .map(|(variant, char)| quote! {
                #char => #ident :: #variant
            });

    let into_char_match_arms = variants_with_chars
        .iter()
        .map(|(variant, char)| (&variant.ident, char))
        .map(|(variant, char)| quote! {
            #ident :: #variant => #char
        });

    quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)] #item_enum

        impl From<char> for #ident {
            fn from(value: char) -> Self {
                match value {
                    #(#from_char_match_arms,)*
                    _ => unreachable!()
                }
            }
        }

        impl Into<char> for #ident {
            fn into(self) -> char {
                match self {
                    #(#into_char_match_arms,)*
                }
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Into::<char>::into(*self))
            }
        }
    }.into()
}