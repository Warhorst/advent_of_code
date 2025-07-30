use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

use colored::Colorize;
use itertools::Itertools;
use proc_macros::execute;

mod y2022;
mod y2023;
mod y2024;
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
                    println!("Solution A: {:?}", solution_a);
                    println!("Solution B: {:?}", solution_b);
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
            .map(Self::parse_text_to_result_and_text::<A>);

        let example_b = read_to_string(format!("./input/{year}/{day}/eb"))
            .ok()
            .map(Self::parse_text_to_result_and_text::<B>);

        let puzzle_solution = match read_to_string(format!("./input/{year}/{day}/s")) {
            Ok(text) => Some(Self::pars_text_to_given_solution(text)),
            Err(_) => None
        };

        Input {
            puzzle_input,
            example_a,
            example_b,
            puzzle_solution
        }
    }

    fn parse_text_to_result_and_text<T: PuzzleResult>(s: String) -> (String, T) {
        // the first line should be the expected result of the example
        let result = s.lines().next().expect("text should not be empty").parse::<T>().expect("first line should be the expected result");
        // use the remaining text as input, but make sure to keep the new lines
        let text = s.lines().skip(1).join("\n");
        (text, result)
    }

    fn pars_text_to_given_solution<T: PuzzleResult, U: PuzzleResult>(s: String) -> (T, U) {
        let mut lines = s.lines();
        let first = lines.next().expect("text should not be empty").parse::<T>().expect("the first line should be the the first solution");
        let second = lines.next().expect("there should be 2 lines").parse::<U>().expect("the second line should be the the second solution");

        (first, second)
    }
}