use std::fmt::Formatter;
use regex::Regex;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .flat_map(|i| i.min_tokens_to_win_a())
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|i| Input {
            button_a: i.button_a,
            button_b: i.button_b,
            prize_location: (10000000000000 + i.prize_location.0, 10000000000000 + i.prize_location.1)
        })
        .flat_map(|i| i.min_tokens_to_win_b())
        .sum()
}

fn parse_input(input: &str) -> Vec<Input> {
    let buttons_regex = Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap();
    let prize_regex = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();

    input
        .split("\n\n")
        .map(|block| {
            let lines = block.lines().collect::<Vec<_>>();

            let button_a = regex_captures(
                lines[0],
                &buttons_regex,
                |caps| (parse(caps[0]), parse(caps[1])),
            ).into_iter().next().unwrap();
            let button_b = regex_captures(
                lines[1],
                &buttons_regex,
                |caps| (parse(caps[0]), parse(caps[1])),
            ).into_iter().next().unwrap();
            let prize_location = regex_captures(
                lines[2],
                &prize_regex,
                |caps| (parse(caps[0]), parse(caps[1])),
            ).into_iter().next().unwrap();

            Input {
                button_a,
                button_b,
                prize_location,
            }
        })
        .collect()
}

struct Input {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize_location: (usize, usize),
}

impl Input {
    fn min_tokens_to_win_a(&self) -> Option<usize> {
        (0..=100)
            .into_iter()
            .flat_map(|a| (0..=100).into_iter().map(move |b| (a, b)))
            .filter(|(a, b)| a * self.button_a.0 + b * self.button_b.0 == self.prize_location.0)
            .filter(|(a, b)| a * self.button_a.1 + b * self.button_b.1 == self.prize_location.1)
            .map(|(a, b)| a * 3 + b)
            .min()
    }

    fn min_tokens_to_win_b(&self) -> Option<usize> {
        println!("{self}");
        println!("{}", self.prize_location.0 / self.button_a.0);
        println!("{}", self.prize_location.0 % self.button_a.0);
        println!("{}", self.prize_location.0 / self.button_b.0);
        println!("{}", self.prize_location.0 % self.button_b.0);
        println!();
        println!();

        None
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A: {:?}, B: {:?}, Prize Location: {:?}", self.button_a, self.button_b, self.prize_location)
    }
}
