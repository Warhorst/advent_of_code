use proc_macros::tile;

use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from(input);

    board
        .tiles_and_positions()
        .filter(|(t, _)| **t == Tile::Roll)
        .filter(|(_, pos)| {
            pos.neighbours()
                .into_iter()
                .filter(|p| board.pos_in_bounds(*p))
                .filter(|p| *board.get_tile(*p).unwrap() == Tile::Roll)
                .count()
                < 4
        })
        .count()
}

pub fn solve_b(input: &str) -> usize {
    let mut board = Board::<Tile>::from(input);
    let mut removed_count = 0;

    loop {
        let removed_positions = board
            .tiles_and_positions()
            .filter(|(t, _)| **t == Tile::Roll)
            .filter(|(_, pos)| {
                pos.neighbours()
                    .into_iter()
                    .filter(|p| board.pos_in_bounds(*p))
                    .filter(|p| *board.get_tile(*p).unwrap() == Tile::Roll)
                    .count()
                    < 4
            })
            .map(|(_, pos)| pos)
            .collect::<Vec<_>>();

        removed_count += removed_positions.len();

        if removed_positions.is_empty() {
            break
        }

        for pos in removed_positions {
            board.set_tile(pos, Tile::Empty).unwrap()
        }
    }

    removed_count
}

#[tile]
enum Tile {
    #[t('@')]
    Roll,
    #[t('.')]
    Empty,
}
