use itertools::Itertools;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::str::FromStr;

mod y2025;

pub fn solve(
    day: u8,
    year: u16,
    run_config: RunConfig,
) -> Option<PuzzleResult> {
    match year {
        2025 => y2025::solve(day, run_config),
        _ => None,
    }
}

pub(crate) fn solve_puzzle<
    A: PuzzleOutput,
    B: PuzzleOutput,
    AS: Fn(&str) -> A,
    BS: Fn(&str) -> B,
>(
    run_config: RunConfig,
    input: Input<A, B>,
    a_solver: AS,
    b_solver: BS,
) -> PuzzleResult {
    let ea_res = if run_config.run_example_a {
        Some(match input.example_a {
            None => ExampleResult::DoesNotExist,
            Some((text, expectation)) => {
                let result = a_solver(&text);

                if expectation == result {
                    ExampleResult::Works
                } else {
                    ExampleResult::Fails {
                        expected: expectation.to_string(),
                        was: result.to_string(),
                    }
                }
            }
        })
    } else {
        None
    };

    let eb_res = if run_config.run_example_b {
        Some(match input.example_b {
            None => ExampleResult::DoesNotExist,
            Some((text, expectation)) => {
                let result = b_solver(&text);

                if expectation == result {
                    ExampleResult::Works
                } else {
                    ExampleResult::Fails {
                        expected: expectation.to_string(),
                        was: result.to_string(),
                    }
                }
            }
        })
    } else {
        None
    };

    let a_res = if run_config.run_a {
        Some(match &input.puzzle_input {
            None => RealResult::DoesNotExist,
            Some(text) => {
                let solution_a = a_solver(text);

                match &input.puzzle_solution {
                    Some((expected_a, _)) => {
                        if &solution_a == expected_a {
                            RealResult::Works(solution_a.to_string())
                        } else {
                            RealResult::Fails {
                                expected: expected_a.to_string(),
                                was: solution_a.to_string(),
                            }
                        }
                    }
                    None => RealResult::Output(solution_a.to_string()),
                }
            }
        })
    } else {
        None
    };

    let b_res = if run_config.run_b {
        Some(match &input.puzzle_input {
            None => RealResult::DoesNotExist,
            Some(text) => {
                let solution_b = b_solver(text);

                match &input.puzzle_solution {
                    Some((_, expected_b)) => {
                        if &solution_b == expected_b {
                            RealResult::Works(solution_b.to_string())
                        } else {
                            RealResult::Fails {
                                expected: expected_b.to_string(),
                                was: solution_b.to_string(),
                            }
                        }
                    }
                    None => RealResult::Output(solution_b.to_string()),
                }
            }
        })
    } else {
        None
    };

    PuzzleResult {
        example_a_result: ea_res,
        a_result: a_res,
        example_b_result: eb_res,
        b_result: b_res,
    }
}

trait PuzzleOutput: Display + PartialEq + FromStr<Err: Debug> {}

impl<T> PuzzleOutput for T where T: Display + PartialEq + FromStr<Err: Debug> {}

pub struct RunConfig {
    pub run_example_a: bool,
    pub run_a: bool,
    pub run_example_b: bool,
    pub run_b: bool,
}

struct Input<A: PuzzleOutput, B: PuzzleOutput> {
    /// Input of the main puzzle
    pub puzzle_input: Option<String>,
    /// Input and expected result of example A, if present
    pub example_a: Option<(String, A)>,
    /// Input and expected result of example B, if present
    pub example_b: Option<(String, B)>,
    /// Existing solutions for A and B. Helpful to automatically check if solvers still work.
    pub puzzle_solution: Option<(A, B)>,
}

impl<A: PuzzleOutput, B: PuzzleOutput> Input<A, B> {
    fn load(
        day: u8,
        year: u16,
    ) -> Self {
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
            Err(_) => None,
        };

        Input {
            puzzle_input,
            example_a,
            example_b,
            puzzle_solution,
        }
    }

    fn parse_example_input<T: PuzzleOutput>(s: String) -> (String, T) {
        // Unify the line endings
        let s = s.replace("\r\n", "\n");
        let mut split = s.split("\n\n");

        // the example solution is the first block before the double new line
        let result = split
            .next()
            .expect("Result and input must be separated by one double new line")
            .parse::<T>()
            .expect("first line should be the expected result");
        // join the remaining block(s) back together using double new lines again
        let text = split.join("\n\n");

        (text, result)
    }

    fn parse_text_to_given_solution<T: PuzzleOutput, U: PuzzleOutput>(s: String) -> (T, U) {
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

pub struct PuzzleResult {
    pub example_a_result: Option<ExampleResult>,
    pub a_result: Option<RealResult>,
    pub example_b_result: Option<ExampleResult>,
    pub b_result: Option<RealResult>,
}

pub enum ExampleResult {
    /// The example does not exist in the input.
    DoesNotExist,
    /// The example output has the expected result.
    Works,
    /// The example output has not the expeted result.
    Fails { expected: String, was: String },
}

pub enum RealResult {
    /// The puzzle input does not exist
    DoesNotExist,
    /// The puzzle has the given output.
    Output(String),
    /// The puzzle has the expected output. Also returns the result.
    Works(String),
    /// The puzzle has not the expected output.
    Fails { expected: String, was: String },
}
