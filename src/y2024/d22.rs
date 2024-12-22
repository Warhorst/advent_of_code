use crate::aoc_lib::*;
use std::collections::HashMap;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse::<usize>(line))
        .map(|num| (0..2000).into_iter().fold(num, |acc, _| calculate_next_number(acc)))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    let last_place = |num: usize| num % 10;

    // todo This produces a slightly off result, and I currently dont know why. The resulting value
    //  is like one price higher than my actual value (I got the star by trial and error).
    //  But the example works...

    input
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
        .flat_map(|(prices, diffs)| diffs
            .windows(4)
            .map(|w| [w[0], w[1], w[2], w[3]])
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, pattern)| {
                if !map.contains_key(&pattern) {
                    map.insert(pattern, prices[i + 3]);
                }
                map
            })
        )
        .fold(HashMap::with_capacity(50000), |mut map, (pattern, price)| {
            map.entry(pattern).and_modify(|val| *val += price).or_insert(price);
            map
        })
        .values()
        .copied()
        .max()
        .unwrap_or_default()
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