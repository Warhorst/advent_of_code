use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

use colored::Colorize;
use itertools::Itertools;
use proc_macros::execute;

mod y2022;
mod y2023;
mod y2024;
mod y2025;
#[allow(dead_code)]
mod aoc_lib;

fn main() {
    execute!();
}

trait PuzzleResult: Debug + PartialEq + FromStr<Err: Debug> {}

impl<T> PuzzleResult for T where T: Debug + PartialEq + FromStr<Err: Debug> {}

fn solve<
    A: PuzzleResult,
    B: PuzzleResult,
    AS: Fn(&str) -> A,
    BS: Fn(&str) -> B
>(
    day: u8,
    year: u16,
    a_solver: AS,
    b_solver: BS,
) {
    println!("Solving day {day}, {year}");

    let input = Input::<A, B>::load_input(day, year);

    match input.example_a {
        None => println!("example A does not exist yet, skipping it"),
        Some((text, expectation)) => {
            let result = a_solver(&text);

            if expectation == result {
                println!("{}", "Example A works".green())
            } else {
                println!("{}", format!("Example A failed. Expected was {expectation:?}, but result was {result:?}.").red());
                return;
            }
        }
    }

    match input.example_b {
        None => println!("example B does not exist yet, skipping it"),
        Some((text, expectation)) => {
            let result = b_solver(&text);

            if expectation == result {
                println!("{}", "Example B works".green())
            } else {
                println!("{}", format!("Example B failed. Expected was {expectation:?}, but result was {result:?}.").red());
                return;
            }
        }
    }

    match input.puzzle_input {
        None => println!("the puzzle input does not exist yet, skipping it"),
        Some(text) => {
            let solution_a = a_solver(&text);
            let solution_b = b_solver(&text);

            match input.puzzle_solution {
                Some((expected_a, expected_b)) => {
                    if expected_a == solution_a {
                       println!("{}", format!("Puzzle A works, returning {expected_a:?} as expected").green())
                    } else {
                       println!("{}", format!("Puzzle A failed. Expected {expected_a:?}, but result was {solution_a:?}").red())
                    }

                    if expected_b == solution_b {
                        println!("{}", format!("Puzzle B works, returning {expected_b:?} as expected").green())
                    } else {
                        println!("{}", format!("Puzzle B failed. Expected {expected_b:?}, but result was {solution_b:?}").red())
                    }
                }
                None => {
                    println!("Solution A: {solution_a:?}");
                    println!("Solution B: {solution_b:?}");
                }
            }
        }
    }
}

struct Input<A: PuzzleResult, B: PuzzleResult> {
    /// Input of the main puzzle
    pub puzzle_input: Option<String>,
    /// Input and expected result of example A, if present
    pub example_a: Option<(String, A)>,
    /// Input and expected result of example B, if present
    pub example_b: Option<(String, B)>,
    pub puzzle_solution: Option<(A, B)>
}

impl<A: PuzzleResult, B: PuzzleResult> Input<A, B> {
    fn load_input(day: u8, year: u16) -> Self {
        let puzzle_input = read_to_string(format!("./input/{year}/{day}/p"))
            .ok()
            .map(|s| s.replace("\r\n", "\n")); // replace line endings to resolve regex issues
        let example_a = read_to_string(format!("./input/{year}/{day}/ea"))
            .ok()
            .map(Self::parse_example_input::<A>);

        let example_b = read_to_string(format!("./input/{year}/{day}/eb"))
            .ok()
            .map(Self::parse_example_input::<B>);

        let puzzle_solution = match read_to_string(format!("./input/{year}/{day}/s")) {
            Ok(text) => Some(Self::parse_text_to_given_solution(text)),
            Err(_) => None
        };

        Input {
            puzzle_input,
            example_a,
            example_b,
            puzzle_solution
        }
    }

    fn parse_example_input<T: PuzzleResult>(s: String) -> (String, T) {
        // Unify the line endings
        let s = s.replace("\r\n", "\n");
        let mut split = s.split("\n\n");

        // the example solution is the first block before the double new line
        let result = split.next().expect("Result and input must be separated by one double new line").parse::<T>().expect("first line should be the expected result");
        // join the remaining block(s) back together using double new lines again
        let text = split.join("\n\n");

        (text, result)
    }

    fn parse_text_to_given_solution<T: PuzzleResult, U: PuzzleResult>(s: String) -> (T, U) {
        // Unify the line endings
        let s = s.replace("\r\n", "\n");
        let mut solutions = s.split("\n\n");

        let first = solutions
            .next()
            .expect("text should not be empty")
            .parse::<T>()
            .expect("The first block should be the first solution");

        let second = solutions
            .next()
            .expect("text should contain 2 blocks")
            .parse::<U>()
            .expect("The second block should be second solution");

        (first, second)
    }
}
