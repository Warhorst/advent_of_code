use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let ordering = split[0];
    let updates = split[1];

    let mut orderings = HashMap::with_capacity(ordering.lines().count());

    for line in ordering.lines() {
        let mut split = line.split("|");
        let a = parse::<usize>(split.next().unwrap());
        let b = parse::<usize>(split.next().unwrap());
        orderings.entry(a).or_insert(HashSet::new()).insert(b);
    }

    updates
        .lines()
        .map(|line| line.split(",").map(|e| parse::<usize>(e)).collect::<Vec<_>>())
        .filter(|update| correctly_ordered(&orderings, update))
        .map(|update| update[update.len() / 2])
        .sum()
}


pub fn solve_b(input: &str) -> usize {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let ordering = split[0];
    let updates = split[1];

    let mut orderings = HashMap::with_capacity(ordering.lines().count());

    for line in ordering.lines() {
        let mut split = line.split("|");
        let a = parse::<usize>(split.next().unwrap());
        let b = parse::<usize>(split.next().unwrap());
        orderings.entry(a).or_insert(HashSet::new()).insert(b);
    }

    updates
        .lines()
        .map(|line| line.split(",").map(|e| parse::<usize>(e)).collect::<Vec<_>>())
        .filter(|update| !correctly_ordered(&orderings, update))
        .map(|mut update| {
            update.sort_by(|a, b| {
                // I don't really know why Greater is the correct negative case here. The puzzle
                // only states that if a rule a|b exists, a is less than b, but not that a is greater than b
                // if no such rule exists. Confusing!
                match a.cmp(b) {
                    Ordering::Equal => Ordering::Equal,
                    _ => match orderings.get(a) {
                        Some(set) => match set.contains(b) {
                            true => Ordering::Less,
                            false => Ordering::Greater
                        }
                        None => Ordering::Greater
                    }
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn correctly_ordered(orderings: &HashMap<usize, HashSet<usize>>, update: &[usize]) -> bool {
    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            match orderings.get(&update[i]) {
                Some(set) => if !set.contains(&update[j]) {
                    return false
                }
                None => return false
            }
        }
    }

    true
}