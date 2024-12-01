use crate::aoc_lib::*;
use std::collections::HashMap;
use regex::Regex;

pub fn solve_a(input: &str) -> isize {
    let num_lines = input.lines().count();

    let mut left = Vec::<isize>::with_capacity(num_lines);
    let mut right = Vec::<isize>::with_capacity(num_lines);
    let regex = Regex::new(r"(?m)^(\d+)\s+(\d+)$").unwrap();

    parse_entries(
        input,
        &regex,
        |caps| (parse(caps[0]), parse(caps[1]))
    )
        .into_iter()
        .for_each(|(l, r)| {
            left.push(l);
            right.push(r);
        });

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn solve_b(input: &str) -> isize {
    let num_lines = input.lines().count();

    let mut left = Vec::with_capacity(num_lines);
    // probably too much capacity, but whatever
    let mut right = HashMap::<isize, isize>::with_capacity(num_lines);
    let regex = Regex::new(r"(?m)^(\d+)\s+(\d+)$").unwrap();

    parse_entries(
        input,
        &regex,
        |caps| (parse(caps[0]), parse(caps[1]))
    )
        .into_iter()
        .for_each(|(l, r)| {
            left.push(l);
            right.entry(r).and_modify(|amount| *amount += 1).or_insert(1);
        });

    left.into_iter()
        .map(|num| match right.get(&num) {
            Some(amount) => num * *amount,
            None => 0
        })
        .sum()
}