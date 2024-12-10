use std::collections::HashSet;
use std::ops::Deref;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from_text(input);
    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(tile, pos)| match tile.0 {
            0 => Some(pos),
            _ => None
        })
        .map(|head_pos| count_trails(&board, head_pos))
        .sum()
}

fn count_trails(board: &Board<Tile>, head: Position) -> usize {
    let mut reached_ends = HashSet::new();
    add_reachable_ends(board, 0, head, &mut reached_ends);
    reached_ends.len()
}

fn add_reachable_ends(
    board: &Board<Tile>,
    current: usize,
    current_pos: Position,
    reached_ends: &mut HashSet<Position>
) {
    if current == 9 {
        reached_ends.insert(current_pos);
    } else {
        current_pos
            .cardinal_neighbours()
            .into_iter()
            .filter_map(|pos| match board.get_tile(pos) {
                Some(tile) => Some((**tile, pos)),
                None => None
            })
            .filter(|(num, _)| *num == (current + 1))
            .for_each(|(num, pos)| add_reachable_ends(board, num, pos, reached_ends))
    }
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<Tile>::from_text(input);
    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(tile, pos)| match tile.0 {
            0 => Some(pos),
            _ => None
        })
        .map(|head_pos| count_unique_trails(&board, head_pos, 0))
        .sum()
}

fn count_unique_trails(board: &Board<Tile>, pos: Position, current: usize) -> usize {
    if current == 9 {
        1
    } else {
        pos
            .cardinal_neighbours()
            .into_iter()
            .filter_map(|pos| match board.get_tile(pos) {
                Some(tile) => Some((**tile, pos)),
                None => None
            })
            .filter(|(num, _)| *num == (current + 1))
            .map(|(num, pos)| count_unique_trails(board, pos, num))
            .sum()
    }
}

struct Tile(usize);

impl Deref for Tile {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile(value.to_digit(10).unwrap() as usize)
    }
}