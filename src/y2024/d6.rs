use std::collections::HashSet;
use crate::aoc_lib::*;
use rayon::iter::ParallelIterator;
use rayon::iter::ParallelBridge;

pub fn solve_a(input: &str) -> usize {
    let (board, guardians) = Board::board_and_specials_from_str(input, |c, pos| match c {
        '^' => Some((Guardian::new(pos), Tile::Free)),
        _ => None
    });

    let mut guardian = guardians[0];
    let mut visited_positions = HashSet::with_capacity(board.len());

    loop {
        visited_positions.insert(guardian.current_position);

        match board.get_tile(guardian.next_pos()) {
            None => return visited_positions.len(),
            Some(tile) => match tile {
                Tile::Free => guardian.go_forward(),
                Tile::Obstacle => guardian.turn()
            }
        }
    }
}

pub fn solve_b(input: &str) -> usize {
    let (board, guardians) = Board::board_and_specials_from_str(input, |c, pos| match c {
        '^' => Some((Guardian::new(pos), Tile::Free)),
        _ => None
    });

    let guardian = guardians[0];

    board
        .positions()
        .par_bridge()
        .map(|obs_pos| {
            // solve with good old brute force
            let mut guard = guardian;
            let mut visited_spots = HashSet::with_capacity(board.len());

            loop {
                let spot = (guard.current_position, guard.current_direction);

                if visited_spots.contains(&spot) {
                    // The guardian already was at this position with this direction,
                    // so she must be in a loop. Return 1 to add it to the count.
                    break 1;
                }

                visited_spots.insert(spot);

                // if the current pos is the new obstacle pos, return
                // an obstacle
                let next_tile = match guard.next_pos() {
                    pos if pos == obs_pos => Some(Tile::Obstacle),
                    _ => board.get_tile(guard.next_pos()).copied()
                };

                match next_tile {
                    // The guardian broke out, no loop here.
                    // Return 0 to add to the count.
                    None => break 0,
                    Some(tile) => match tile {
                        Tile::Free => guard.go_forward(),
                        Tile::Obstacle => guard.turn()
                    }
                }
            }
        })
        .sum()
}

#[derive(Clone, Copy)]
struct Guardian {
    current_position: Position,
    current_direction: Direction,
}

impl Guardian {
    fn new(pos: Position) -> Self {
        Guardian {
            current_position: pos,
            current_direction: YM,
        }
    }

    fn next_pos(&self) -> Position {
        self.current_position.position_in_direction(self.current_direction, 1)
    }

    fn go_forward(&mut self) {
        self.current_position = self.next_pos()
    }

    fn turn(&mut self) {
        self.current_direction = match self.current_direction {
            XP => YP,
            XM => YM,
            YP => XM,
            YM => XP,
            _ => panic!("unexpected dir")
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Free,
    Obstacle,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => Tile::Free,
            '#' => Tile::Obstacle,
            _ => panic!("unknown: {value}")
        }
    }
}