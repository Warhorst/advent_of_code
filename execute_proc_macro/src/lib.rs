use proc_macro::TokenStream;
use std::fs::read_dir;
use std::path::Path;

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
    let mut code = String::new();

    code += "let args: Vec<String> = std::env::args().collect();";
    code += "let year = args[1].parse::<usize>().unwrap();";
    code += "let day = args[2].parse::<usize>().unwrap();";
    code += "match (year, day) {";

    for (y, d) in load_years_and_days() {
        code += format!("(y, d) if year == {y} && day == {d} => solve({d}, {y}, crate::y{y}::d{d}::solve_a, crate::y{y}::d{d}::solve_b),").as_str();
    }
    code += "(y, d) => panic!(\"there is no implementation for year {} and day {}\", y, d)";
    code += "}";

    code.parse().unwrap()
}

fn load_years_and_days() -> impl IntoIterator<Item=(usize, usize)> {
    let mut years_and_days = vec![];
    let path = Path::new("./src");

    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() && path_name.starts_with("y") {
            let year = path_name.replace("y", "").parse::<usize>().unwrap();

            for entry in read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let path_name = path.file_name().unwrap().to_str().unwrap();

                if path.is_file() && path_name.starts_with("d") {
                    let day = path_name
                        .replace("d", "")
                        .replace(".rs", "")
                        .parse::<usize>()
                        .unwrap();

                    years_and_days.push((year, day))
                }
            }
        }
    }

    years_and_days
}