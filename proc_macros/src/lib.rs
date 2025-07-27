mod from_regex;
mod execute;
mod tile;

use proc_macro::TokenStream;

/// This macro generates and executes code to run the puzzle for the given day.
/// It does the following steps:
/// - read the year and day from the program arguments
/// - read every folder starting with "y" and every file in it starting with "d" to identify puzzle solutions
/// - parse the findings to a giant match expression which just searches for the puzzle matching the user input
///
/// A puzzle solution file must contain 2 public functions named "solve_a" and "solve_b" which take
/// a string reference and return something that satisfies the PuzzleResult trait in main.
///
/// This way, I just need to create new puzzle solutions and never need to touch main again.
#[proc_macro]
pub fn execute(_item: TokenStream) -> TokenStream {
    execute::create()
}

/// Generates implementations for unit like enums to be used as tiles in a Board.
/// Generates From<char>, Into<char> and Display. Adds derives for Clone, Copy, Debug, Eq, Hash and PartialEq.
/// The char to use is specified by the t-attribute, like #[t('#')]
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
