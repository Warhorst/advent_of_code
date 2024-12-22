use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse::<usize>(line))
        .map(|num| (0..2000).into_iter().fold(num, |acc, _| calculate_next_number(acc)))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let last_place = |num: usize| num % 10;

    let collect = input
        .lines()
        .map(|line| parse::<usize>(line))
        .map(|num| (0..2000)
            .into_iter()
            .fold((num, vec![last_place(num)], vec![0isize]), |(current_num, mut prices, mut diffs), _| {
                let next = calculate_next_number(current_num);
                prices.push(last_place(next));
                diffs.push(prices[prices.len() - 1] as isize - prices[prices.len() - 2] as isize);
                (next, prices, diffs)
            })
        )
        .map(|(_, prices, diffs)| (prices, diffs))
        .collect::<Vec<_>>();

    let mut max = 0;

    for (_, diffs) in &collect {
        max = usize::max(
            max,
            diffs
                .windows(4)
                .map(|pattern| collect
                    .iter()
                    .flat_map(move |(other_prices, other_diffs)| other_diffs
                        .windows(4)
                        .enumerate()
                        .find(|(_, w)| *w == pattern)
                        .map(|(j, _)| other_prices[j + 3])
                    )
                    .sum()
                )
                .max()
                .unwrap_or_default(),
        );
    }

    max
}

fn calculate_next_number(current: usize) -> usize {
    let a = prune(mix(current * 64, current));
    let b = prune(mix(a / 32, a));
    let c = prune(mix(b * 2048, b));
    c
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(num: usize) -> usize {
    num % 16777216
}