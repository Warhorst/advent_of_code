use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from(input);
    // group positions by frequency
    let mut freq_pos_map = HashMap::new();
    board
        .tiles_and_positions()
        .filter_map(|(tile, pos)| match tile {
            Tile::Antenna(freq) => Some((*freq, pos)),
            Tile::Empty => None
        })
        .for_each(|(freq, pos)| freq_pos_map.entry(freq).or_insert(vec![]).push(pos));

    let mut antinodes = HashSet::new();

    freq_pos_map
        .values()
        // create permutations of each positions with each other position
        .flat_map(|poss| poss.iter().permutations(2))
        .for_each(|perm| {
            let pos = *perm[0];
            let other = *perm[1];
            let next = other + (other - pos);

            if board.pos_in_bounds(next) {
                antinodes.insert(next);
            }
        });

    antinodes.len()
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<Tile>::from(input);
    // group positions by frequency
    let mut freq_pos_map = HashMap::new();
    board
        .tiles_and_positions()
        .filter_map(|(tile, pos)| match tile {
            Tile::Antenna(freq) => Some((*freq, pos)),
            Tile::Empty => None
        })
        .for_each(|(freq, pos)| freq_pos_map.entry(freq).or_insert(vec![]).push(pos));

    let mut antinodes = HashSet::new();

    freq_pos_map
        .values()
        // create permutations of each positions with each other position
        .flat_map(|poss| poss.iter().permutations(2))
        .for_each(|perm| {
            // same as part A, but add positions to the antennas as far as the position is on
            // the board
            let pos = *perm[0];
            let other = *perm[1];
            let diff = other - pos;

            // as part B requires, the antennas are also antinodes themselves
            antinodes.insert(pos);
            antinodes.insert(other);

            let mut count = 1;
            loop {
                let next = other + diff * count;

                if board.pos_in_bounds(next) {
                    antinodes.insert(next);
                    count += 1;
                } else {
                    break
                }
            }
        });


    antinodes.len()
}

#[derive(Clone, Copy)]
enum Tile {
    Antenna(char),
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            c => Tile::Antenna(c)
        }
    }
}
