use itertools::Itertools;

use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = lines
        .pop()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Ops::from)
        .collect::<Vec<_>>();

    let nums = lines
        .iter()
        .map(|line| {
            line.split(|c: char| c.is_whitespace())
                .filter(|part| !part.is_empty())
                .map(parse::<usize>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sums = vec![0; ops.len()];

    for i in 0..ops.len() {
        sums[i] = match ops[i] {
            Ops::Add => nums.iter().map(|ns| ns[i]).sum(),
            Ops::Mul => nums.iter().map(|ns| ns[i]).product(),
        }
    }

    sums.iter().sum()
}

pub fn solve_b(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = lines
        .pop()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Ops::from)
        .collect::<Vec<_>>();

    let board = Board::<char>::from(
        input
            .lines()
            .take(lines.len())
            .join("\n")
            .as_str(),
    );

    let mut index = ops.len();
    let mut nums = vec![vec![]; ops.len()];

    for i in (0..lines[0].len()).rev() {
        let start = p!(i, 0);
        let line = board.line_to_border(start, Direction::YP).unwrap();
        let string = line
            .filter_map(|(_, c)| if !c.is_whitespace() { Some(c) } else { None })
            .collect::<String>();

        if string.is_empty() {
            index -=1;
            continue;
        }

        let num = parse::<usize>(&string);

        nums[index - 1].push(num);
    }

    let mut sum = vec![0; ops.len()];

    for i in 0..ops.len() {
        sum[i] = match ops[i] {
            Ops::Add => nums[i].iter().sum(),
            Ops::Mul => nums[i].iter().product(),
        }
    }

    sum.iter().sum()
}

#[derive(Debug)]
enum Ops {
    Add,
    Mul,
}

impl From<char> for Ops {
    fn from(value: char) -> Self {
        match value {
            '+' => Ops::Add,
            '*' => Ops::Mul,
            _ => panic!("Unknown op"),
        }
    }
}
