use std::collections::HashMap;

pub fn solve_a(input: &str) -> isize {
    let num_lines = input.lines().count();

    let mut left = Vec::with_capacity(num_lines);
    let mut right = Vec::with_capacity(num_lines);

    for line in input.lines() {
        let split = line.trim().split("  ").collect::<Vec<_>>();
        left.push(split[0].parse::<isize>().unwrap());
        right.push(split[1].trim().parse::<isize>().unwrap());
    }

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

    for line in input.lines() {
        let split = line.trim().split("  ").collect::<Vec<_>>();
        left.push(split[0].parse::<isize>().unwrap());

        right.entry(split[1].trim().parse::<isize>().unwrap())
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
    }

    left.into_iter()
        .map(|num| match right.get(&num) {
            Some(amount) => num * *amount,
            None => 0
        })
        .sum()
}