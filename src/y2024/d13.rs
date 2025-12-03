use std::fmt::Formatter;
use regex::Regex;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .flat_map(|i| i.min_tokens_to_win_a())
        .sum()
}

pub fn solve_b(input: &str) -> u128 {
    parse_input(input)
        .into_iter()
        .map(|i| Input {
            button_a: i.button_a,
            button_b: i.button_b,
            prize_location: (10000000000000 + i.prize_location.0, 10000000000000 + i.prize_location.1)
        })
        .map(|i| i.min_tokens_to_win_b())
        .sum::<u128>()
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
            .flat_map(|a| (0..=100).map(move |b| (a, b)))
            .filter(|(a, b)| a * self.button_a.0 + b * self.button_b.0 == self.prize_location.0)
            .filter(|(a, b)| a * self.button_a.1 + b * self.button_b.1 == self.prize_location.1)
            .map(|(a, b)| a * 3 + b)
            .min()
    }

    fn min_tokens_to_win_b(&self) -> u128 {
        // as I didn't had the energy to invest hours into refreshing my algebra knowledge,
        // I used this explanation/formula from this nice person on reddit:
        // https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
        let a0 = self.button_a.0 as isize;
        let a1 = self.button_a.1 as isize;
        let b0 = self.button_b.0 as isize;
        let b1 = self.button_b.1 as isize;
        let p0 = self.prize_location.0 as isize;
        let p1 = self.prize_location.1 as isize;

        let a_presses = ((p0 * b1 - p1 * b0) / (a0 * b1 - a1 * b0)).abs();
        let b_presses = ((p0 * a1 - p1 * a0) / (a0 * b1 - a1 * b0)).abs();

        if (a0 * a_presses + b0 * b_presses, a1 * a_presses + b1 * b_presses) == (p0, p1) {
            (a_presses * 3 + b_presses) as u128
        } else {
            0
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A: {:?}, B: {:?}, Prize Location: {:?}", self.button_a, self.button_b, self.prize_location)
    }
}
