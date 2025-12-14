mod from_regex;
mod tile;

// todo cool idea: create a memoize macro which transforms a function into a memoized version (and maybe apply this macro on 2024 d11)

use proc_macro::TokenStream;

/// Generates implementations for unit like enums to be used as tiles in a Board.
/// Generates From<char>, Into<char> (also for references) and Display. Adds derives for Clone, Copy, Debug, Eq, Hash and PartialEq.
/// The char to use is specified by the t-attribute, like #[t('#')].
///
/// Example:
///
/// ```
/// #[tile]
/// enum ChessPiece {
///     #[t('K')]
///     King,
///     #[t('P')]
///     Pawn,
///     #[t('R')]
///     Rook
/// }
/// ```
#[proc_macro_attribute]
pub fn tile(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    tile::create(item)
}

/// Generates a method, from_regex, which creates an instance of the struct/enum with this
/// attribute. This method takes a &str haystack and creates the instance based on regexes which might contain captures.
/// The method will panic if the regex is wrong (not enough captures or captures returning the wrong type)
/// or if none of the regexes match the given haystack. Requires the [regex crate](https://docs.rs/regex/latest/regex/).
///
/// Usage on structs:
/// ```
///
/// #[proc_macros::from_regex]
/// #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
/// struct Named {
///     num: usize,
///     string: String
/// }
///
/// #[proc_macros::from_regex]
/// #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
/// struct Unnamed(usize, String);
///
/// #[proc_macros::from_regex]
/// #[reg(r#"C"#)]
/// struct Unit;
/// ```
///
/// Usage on enums:
/// ```
/// #[proc_macros::from_regex]
/// enum Enum {
///     #[reg(r#"A (\d+) ([a-zA-Z]+)"#)]
///     A {
///         num: usize,
///         string: String
///     },
///     #[reg(r#"B (\d+) ([a-zA-Z]+)"#)]
///     B(usize, String),
///     #[reg(r#"C"#)]
///     C
/// }
/// ```
///
/// Expects the struct / every enum variant to have the reg attribute, which must have a single string
/// literal as parameter.
///
/// Important: The regex structs are generated as static LazyLock instances, which means they are initialized only once.
#[proc_macro_attribute]
pub fn from_regex(_attr: TokenStream, input: TokenStream) -> TokenStream {
    from_regex::create(input)
}
