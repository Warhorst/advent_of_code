use helpers::prelude::*;
use std::collections::HashMap;

pub fn solve_a(input: &str) -> usize {
    input
        .lines()
        .map(parse::<usize>)
        .map(|num| (0..2000).fold(num, |acc, _| calculate_next_number(acc)))
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    // the price is the first digit of the given number
    let get_price = |num: usize| num % 10;

    input
        .lines()
        .map(parse::<usize>)
        // calculate the next 2000 numbers, prices and the diffs
        .map(|num| (0..2000)
            .fold((num, vec![get_price(num)], vec![]), |(current_num, mut prices, mut diffs), _| {
                let next = calculate_next_number(current_num);
                prices.push(get_price(next));
                diffs.push(prices[prices.len() - 1] as isize - prices[prices.len() - 2] as isize);
                (next, prices, diffs)
            })
        )
        // the number is no longer needed, keep only the prices and diffs
        .map(|(_, prices, diffs)| (prices, diffs))
        // get all the first occurrences of a pattern in the diffs, alongside their price
        .flat_map(|(prices, diffs)| diffs
            .windows(4)
            .map(|w| [w[0], w[1], w[2], w[3]])
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, pattern)| {
                // use this map to only use the first occurrence
                // the price index is 3 (the index of the end of the window) + 1 (because the diffs are one element shorter than the prices)
                map.entry(pattern).or_insert(prices[i + 4]);
                map
            })
        )
        // collect each occurred pattern and sum the prices for them
        .fold(HashMap::with_capacity(50000), |mut map, (pattern, price)| {
            map.entry(pattern).and_modify(|val| *val += price).or_insert(price);
            map
        })
        // return the max price sum
        .values()
        .copied()
        .max()
        .unwrap_or_default()
}

fn calculate_next_number(current: usize) -> usize {
    let a = prune(mix(current * 64, current));
    let b = prune(mix(a / 32, a));
    prune(mix(b * 2048, b))
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(num: usize) -> usize {
    num % 16777216
}
