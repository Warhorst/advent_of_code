use itertools::Itertools;
use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| block.lines().map(parse::<usize>).sum())
        .max()
        .unwrap()
}

pub fn solve_b(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| block.lines().map(parse::<usize>).sum())
        .sorted_by(|a: &usize, b: &usize| b.cmp(a))
        .take(3)
        .sum()
}
