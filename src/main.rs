use std::fmt::Debug;
use std::fs::read_to_string;

use colored::Colorize;

use crate::y2023::d1::{solve_1a, solve_1b};
use crate::y2023::d10::{solve_10a, solve_10b};
use crate::y2023::d11::{solve_11a, solve_11b};
use crate::y2023::d12::{solve_12a, solve_12b};
use crate::y2023::d13::{solve_13a, solve_13b};
use crate::y2023::d14::{solve_14a, solve_14b};
use crate::y2023::d15::{solve_15a, solve_15b};
use crate::y2023::d16::{solve_16a, solve_16b};
use crate::y2023::d17::{solve_17a, solve_17b};
use crate::y2023::d18::{solve_18a, solve_18b};
use crate::y2023::d2::{solve_2a, solve_2b};
use crate::y2023::d3::{solve_3a, solve_3b};
use crate::y2023::d4::{solve_4a, solve_4b};
use crate::y2023::d5::{solve_5a, solve_5b};
use crate::y2023::d6::{solve_6a, solve_6b};
use crate::y2023::d7::{solve_7a, solve_7b};
use crate::y2023::d8::{solve_8a, solve_8b};
use crate::y2023::d9::{solve_9a, solve_9b};

mod y2023;
#[allow(dead_code)]
mod aoc_lib;

fn main() {
    solve_day(18)
}

fn solve_day(day: usize) {
    let solve_day_funcs = [
        || solve(1, 2023, solve_1a, 142, solve_1b, 281),
        || solve(2, 2023, solve_2a, 8, solve_2b, 2286),
        || solve(3, 2023, solve_3a, 4361, solve_3b, 467835),
        || solve(4, 2023, solve_4a, 13, solve_4b, 30),
        || solve(5, 2023, solve_5a, 35, solve_5b, 46),
        || solve(6, 2023, solve_6a, 288, solve_6b, 71503),
        || solve(7, 2023, solve_7a, 6440, solve_7b, 5905),
        || solve(8, 2023, solve_8a, 6, solve_8b, 6),
        || solve(9, 2023, solve_9a, 114, solve_9b, 2),
        || solve(10, 2023, solve_10a, 8, solve_10b, 10),
        || solve(11, 2023, solve_11a, 374, solve_11b, 82000210),
        || solve(12, 2023, solve_12a, 21, solve_12b, 525152),
        || solve(13, 2023, solve_13a, 405, solve_13b, 0),
        || solve(14, 2023, solve_14a, 136, solve_14b, 64),
        || solve(15, 2023, solve_15a, 1320, solve_15b, 145),
        || solve(16, 2023, solve_16a, 46, solve_16b, 51),
        || solve(17, 2023, solve_17a, 102, solve_17b, 94),
        || solve(18, 2023, solve_18a, 62, solve_18b, 952408144115),
    ];

    solve_day_funcs[day - 1]()
}

fn solve<A: Debug + PartialEq, B: Debug + PartialEq, AS: Fn(&str) -> A, BS: Fn(&str) -> B>(
    day: u8,
    year: u16,
    a_solver: AS,
    a_expectation: A,
    b_solver: BS,
    b_expectation: B,
) {
    println!("Solving day {day}, {year}");

    let input = load_input(day, year);

    if input.example_a.is_empty() {
        println!("example A does not exist yet, skipping it");
    } else {
        let a_result = a_solver(&input.example_a);
        if a_expectation == a_result {
            println!("{}", "Example A works".green())
        } else {
            println!("{}", format!("Example A failed. Expected was {:?}, but result was {:?}.", a_expectation, a_result).red());
            return;
        }
    }

    if input.example_b.is_empty() {
        println!("example B does not exist yet, skipping it");
    } else {
        let b_result = b_solver(&input.example_b);

        if b_expectation == b_result {
            println!("{}", "Example B works".green())
        } else {
            println!("{}", format!("Example B failed. Expected was {:?}, but result was {:?}.", b_expectation, b_result).red());
            return;
        }
    }

    if input.puzzle_input.is_empty() {
        println!("the puzzle input does not exist yet, skipping it");
    } else {
        println!("Solution A: {:?}", a_solver(&input.puzzle_input));
        println!("Solution B: {:?}", b_solver(&input.puzzle_input));
    }
}

pub struct Input {
    pub puzzle_input: String,
    pub example_a: String,
    pub example_b: String
}

pub fn load_input(day: u8, year: u16) -> Input {
    let puzzle_input = read_to_string(format!("./input/{year}/{day}/p.txt")).unwrap_or(String::default());
    let example_a = read_to_string(format!("./input/{year}/{day}/ea.txt")).unwrap_or(String::default());
    let example_b = read_to_string(format!("./input/{year}/{day}/eb.txt")).unwrap_or(String::default());

    Input {
        puzzle_input,
        example_a,
        example_b
    }
}