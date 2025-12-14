use regex::Regex;
use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .flat_map(|line| regex_captures(
            line,
            &regex,
            |caps| (
                (parse::<usize>(caps[0]), parse::<usize>(caps[1])),
                (parse::<usize>(caps[2]), parse::<usize>(caps[3]))
            ),
        ))
        .filter(|((a0, a1), (b0, b1))| {
            let range_a = a0..=a1;
            let range_b = b0..=b1;
            range_a.contains(&b0) && range_a.contains(&b1) || range_b.contains(&a0) && range_b.contains(&a1)
        })
        .count()
}

pub fn solve_b(input: &str) -> usize {
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .flat_map(|line| regex_captures(
            line,
            &regex,
            |caps| (
                (parse::<usize>(caps[0]), parse::<usize>(caps[1])),
                (parse::<usize>(caps[2]), parse::<usize>(caps[3]))
            ),
        ))
        .filter(|((a0, a1), (b0, b1))| {
            let range_a = a0..=a1;
            let range_b = b0..=b1;
            range_a.contains(&b0) || range_a.contains(&b1) || range_b.contains(&a0) || range_b.contains(&a1)
        })
        .count()
}
