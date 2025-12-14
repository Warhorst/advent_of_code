use aoc_solvers::{ExampleResult, RealResult, RunConfig};
use colored::Colorize;

/// Execute a puzzle for a given year and day.
///
/// This binary takes 3 inputs:
/// 1. The year (required): The year of the puzzle
/// 2. The day (required): The day of the puzzle
/// 3. The selection (optional): A string which contains a space separated list of selection identifiers. If not set, every example and puzzle will be executed. Possible values:
///     - ea -> Run example A
///     - eb -> Run example B
///     - a -> Run puzzle A
///     - b -> Run puzzle B
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let year = args[1].parse::<usize>().unwrap();
    let day = args[2].parse::<usize>().unwrap();

    let selection_string = args.get(3);

    let run_config = match selection_string {
        Some(selection) => {
            let parts = selection.split(" ").collect::<Vec<_>>();

            RunConfig {
                run_example_a: parts.contains(&"ea"),
                run_a: parts.contains(&"a"),
                run_example_b: parts.contains(&"eb"),
                run_b: parts.contains(&"b"),
            }
        }
        // No part string is provided, so every example and puzzle will be executed
        None => RunConfig {
            run_example_a: true,
            run_a: true,
            run_example_b: true,
            run_b: true,
        },
    };

    let res = match aoc_solvers::solve(day as u8, year as u16, run_config) {
        Some(r) => r,
        None => panic!("No solver exist for year {year} and day {day}!"),
    };

    if let Some(ea) = res.example_a_result {
        match ea {
            ExampleResult::DoesNotExist => println!("Example A does not exist yet, skipping it"),
            ExampleResult::Works => println!("{}", "Example A works".green()),
            ExampleResult::Fails { expected, was } => println!(
                "{}",
                format!("Example A failed. Expected was {expected}, but result was {was}.").red()
            ),
        }
    }

    if let Some(eb) = res.example_b_result {
        match eb {
            ExampleResult::DoesNotExist => println!("Example B does not exist yet, skipping it"),
            ExampleResult::Works => println!("{}", "Example B works".green()),
            ExampleResult::Fails { expected, was } => println!(
                "{}",
                format!("Example B failed. Expected was {expected}, but result was {was}.").red()
            ),
        }
    }

    if let Some(a) = res.a_result {
        match a {
            RealResult::DoesNotExist => {
                println!("The puzzle input does not exist yet, skipping it")
            }
            RealResult::Output(solution) => println!("Solution A: {solution}"),
            RealResult::Works(val) => println!(
                "{}",
                format!("Puzzle A works, returning {val} as expected").green()
            ),
            RealResult::Fails { expected, was } => println!(
                "{}",
                format!("Puzzle A failed. Expected {expected}, but result was {was}").red()
            ),
        }
    }

    if let Some(b) = res.b_result {
        match b {
            RealResult::DoesNotExist => {
                println!("The puzzle input does not exist yet, skipping it")
            }
            RealResult::Output(solution) => println!("Solution B: {solution}"),
            RealResult::Works(val) => println!(
                "{}",
                format!("Puzzle B works, returning {val} as expected").green()
            ),
            RealResult::Fails { expected, was } => println!(
                "{}",
                format!("Puzzle B failed. Expected {expected}, but result was {was}").red()
            ),
        }
    }
}
