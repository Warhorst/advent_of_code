use crate::aoc_lib::*;
use std::collections::HashMap;

pub fn solve_a(input: &str) -> usize {
    let mut cache = HashMap::new();

    input
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(parse::<usize>)
        .map(|num| get_resulting_amount(num, 25, &mut cache))
        .sum::<usize>()
}

/// From Past-Me: The trick to solving this one was to ask myself "how many different numbers do actually occur?",
/// which the answer was "not so many!". A single number like 125 just produces around 50 different numbers.
/// This is small enough to just cache results.
pub fn solve_b(input: &str) -> u128 {
    let mut cache = HashMap::new();

    input
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(parse::<usize>)
        .map(|num| get_resulting_amount(num, 75, &mut cache))
        .fold(0u128, |acc, item| acc + item as u128)
}

fn get_resulting_amount(num: usize, current_depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let amount_opt = cache.get(&(num, current_depth)).copied();

    match amount_opt {
        // the number was already calculated for this depth, return the amount
        Some(amount) => amount,
        // no cached value exists
        None => {
            // calculate the value based on the rules
            let new = if current_depth == 0 {
                1
            } else {
                match num {
                    0 => get_resulting_amount(1, current_depth - 1, cache),
                    n => match try_split_num(n) {
                        Some((a, b)) => get_resulting_amount(a, current_depth - 1, cache) + get_resulting_amount(b, current_depth - 1, cache),
                        None => get_resulting_amount(n * 2024, current_depth - 1, cache)
                    }
                }
            };

            // save it in the cache
            cache.insert((num, current_depth), new);

            new
        }
    }
}

fn try_split_num(num: usize) -> Option<(usize, usize)> {
    let places = num.checked_ilog10().unwrap_or(0) + 1;

    if places.is_multiple_of(2) {
        let pow = 10_usize.pow(places / 2);
        let left = num / pow;
        let right = num % pow;

        Some((left, right))
    } else {
        None
    }
}
