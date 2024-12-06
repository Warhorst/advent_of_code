use std::collections::HashSet;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let board = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let guardian_pos = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("^"))
        .find_map(|(y, line)| line.chars().enumerate().find_map(|(x, char)| match char {
            '^' => Some(p!(x, y)),
            _ => None
        }))
        .expect("The guardian must exist");

    let mut guardian = Guardian::new(guardian_pos);
    let mut visited_positions = HashSet::with_capacity(board.len() * board[0].len());

    loop {
        visited_positions.insert(guardian.current_position);

        match get_tile(&board, guardian.next_pos()) {
            None => return visited_positions.len(),
            Some(tile) => match tile {
                Tile::Free => guardian.go_forward(),
                Tile::Obstacle => guardian.turn()
            }
        }
    }
}

fn get_tile(board: &Vec<Vec<Tile>>, pos: Position) -> Option<Tile> {
    board.get(pos.y as usize)?.get(pos.x as usize).copied()
}

pub fn solve_b(input: &str) -> usize {
    let board = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let guardian_pos = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("^"))
        .find_map(|(y, line)| line.chars().enumerate().find_map(|(x, char)| match char {
            '^' => Some(p!(x, y)),
            _ => None
        }))
        .expect("The guardian must exist");

    let mut count = 0;
    for obs_pos in p!(0, 0).iter_to(p!(board[0].len() - 1, board.len() - 1)) {
        // solve with good old brute force
        let mut guardian = Guardian::new(guardian_pos);
        let mut visited_spots = HashSet::with_capacity(board.len() * board[0].len());

        loop {
            let spot = (guardian.current_position, guardian.current_direction);

            if visited_spots.contains(&spot) {
                count += 1;
                break
            }

            visited_spots.insert(spot);

            let next_tile = match guardian.next_pos() {
                pos if pos == obs_pos => Some(Tile::Obstacle),
                _ => get_tile(&board, guardian.next_pos())
            };

            match next_tile {
                None => break,
                Some(tile) => match tile {
                    Tile::Free => guardian.go_forward(),
                    Tile::Obstacle => guardian.turn()
                }
            }
        }

    }

    count
}

struct Guardian {
    current_position: Position,
    current_direction: Direction
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

#[derive(Copy, Clone)]
enum Tile {
    Free,
    Obstacle
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