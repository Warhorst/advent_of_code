use aoc_solvers::{ExampleResult, RealResult, RunConfig};
use colored::Colorize;

#[allow(dead_code)]
mod aoc_lib;
mod y2024;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let year = args[1].parse::<usize>().unwrap();
    let day = args[2].parse::<usize>().unwrap();
    let run_config = RunConfig {
        run_example_a: true,
        run_a: true,
        run_example_b: true,
        run_b: true,
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
