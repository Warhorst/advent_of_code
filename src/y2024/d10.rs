use crate::aoc_lib::*;
use std::collections::HashSet;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<usize>::from_text_and_mapping(input, |c| c.to_digit(10).unwrap() as usize);
    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(num, pos)| match num {
            0 => Some(pos),
            _ => None
        })
        .map(|head_pos| count_trails(&board, head_pos))
        .sum()
}

fn count_trails(board: &Board<usize>, head: Position) -> usize {
    let mut reached_ends = HashSet::new();
    add_reachable_ends(board, 0, head, &mut reached_ends);
    reached_ends.len()
}

fn add_reachable_ends(
    board: &Board<usize>,
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
                Some(num) => Some((*num, pos)),
                None => None
            })
            .filter(|(num, _)| *num == (current + 1))
            .for_each(|(num, pos)| add_reachable_ends(board, num, pos, reached_ends))
    }
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<usize>::from_text_and_mapping(input, |c| c.to_digit(10).unwrap() as usize);
    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(num, pos)| match num {
            0 => Some(pos),

            _ => None
        })
        .map(|head_pos| count_unique_trails(&board, head_pos, 0))
        .sum()
}

fn count_unique_trails(board: &Board<usize>, pos: Position, current: usize) -> usize {
    if current == 9 {
        1
    } else {
        pos
            .cardinal_neighbours()
            .into_iter()
            .filter_map(|pos| match board.get_tile(pos) {
                Some(num) => Some((*num, pos)),
                None => None
            })
            .filter(|(num, _)| *num == (current + 1))
            .map(|(num, pos)| count_unique_trails(board, pos, num))
            .sum()
    }
}