use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

use colored::Colorize;
use itertools::Itertools;
use execute_proc_macro::execute;

mod y2023;
mod y2024;
#[allow(dead_code)]
mod aoc_lib;

fn main() {
    execute!();
}

trait PuzzleResult : Debug + PartialEq + FromStr<Err: Debug> + Debug {}

impl<T> PuzzleResult for T where T: Debug + PartialEq + FromStr<Err: Debug> + Debug {}

fn solve<
    A: PuzzleResult,
    B: PuzzleResult,
    AS: Fn(&str) -> A,
    BS: Fn(&str) -> B
>(
    day: u8,
    year: u16,
    a_solver: AS,
    b_solver: BS
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
                println!("{}", format!("Example A failed. Expected was {:?}, but result was {:?}.", expectation, result).red());
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
                println!("{}", format!("Example B failed. Expected was {:?}, but result was {:?}.", expectation, result).red());
                return;
            }
        }
    }

    match input.puzzle_input {
        None => println!("the puzzle input does not exist yet, skipping it"),
        Some(text) => {
            println!("Solution A: {:?}", a_solver(&text));
            println!("Solution B: {:?}", b_solver(&text));
        }
    }
}

struct Input<A: PuzzleResult, B: PuzzleResult> {
    /// Input of the main puzzle
    pub puzzle_input: Option<String>,
    /// Input and expected result of example A, if present
    pub example_a: Option<(String, A)>,
    /// Input and expected result of example B, if present
    pub example_b: Option<(String, B)>
}

impl<A: PuzzleResult, B: PuzzleResult> Input<A, B> {
    fn load_input(day: u8, year: u16) -> Self {
        let puzzle_input = read_to_string(format!("./input/{year}/{day}/p.txt")).ok();
        let example_a = read_to_string(format!("./input/{year}/{day}/ea.txt"))
            .ok()
            .map(Self::parse_text_to_result_and_text::<A>);

        let example_b = read_to_string(format!("./input/{year}/{day}/eb.txt"))
            .ok()
            .map(Self::parse_text_to_result_and_text::<B>);

        Input {
            puzzle_input,
            example_a,
            example_b
        }
    }

    fn parse_text_to_result_and_text<T: PuzzleResult>(s: String) -> (String, T) {
        // the first line should be the expected result of the example
        let result = s.lines().next().expect("text should not be empty").parse::<T>().expect("first line should be the expected result");
        // use the remaining text as input, but make sure to keep the new lines
        let text = s.lines().skip(1).join("\n");
        (text, result)
    }
}