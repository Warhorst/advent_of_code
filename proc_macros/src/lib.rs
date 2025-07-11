mod from_regex;
mod execute;

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

#[proc_macro_attribute]
pub fn from_regex(_attr: TokenStream, input: TokenStream) -> TokenStream {
    from_regex::create(input)
}
