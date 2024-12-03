use std::str::FromStr;
use regex::Regex;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    input
        .lines()
        .flat_map(|line| regex_captures(
            line,
            &regex,
            |caps| (parse::<usize>(caps[0]), parse::<usize>(caps[1]))
        ))
        .map(|(a, b)| a * b)
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let regex = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    multiply(input
        .lines()
        .flat_map(|line| regex_captures(
            line,
            &regex,
            |caps| Ins::from_str(caps[0]).unwrap()
        ))
    )
}

#[derive(Clone, Copy, Debug)]
enum Ins {
    Mul(usize, usize),
    Do,
    Dont
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Ins::*;

        if s.starts_with("mul") {
            let cleaned = s.replace("mul(", "").replace(")", "");
            let split = cleaned.split(",").collect::<Vec<_>>();
            Ok(Mul(parse(split[0]), parse(split[1])))
        } else if s.starts_with("don't") {
            Ok(Dont)
        } else if s.starts_with("do") {
            Ok(Do)
        } else {
            Err(format!("could not parse {s}"))
        }
    }
}

fn multiply(instructions: impl IntoIterator<Item=Ins>) -> usize {
    let mut sum = 0;
    // is multiplying currently enabled?
    let mut is_do = true;

    for ins in instructions {
        match ins {
            Ins::Mul(a, b) => if is_do { sum += a * b}
            Ins::Do => is_do = true,
            Ins::Dont => is_do = false
        }
    }

    sum
}