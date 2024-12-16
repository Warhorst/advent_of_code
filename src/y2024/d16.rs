use std::collections::HashSet;
use crate::aoc_lib::*;
use Tile::*;
use pathfinding::prelude::astar;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from_text(input);
    let start = (board.get_position_of(&Start).unwrap(), XP);
    let goal = board.get_position_of(&End).unwrap();

    let (_path, cost) = astar(
        &start,
        |(pos, dir)| pos.cardinal_neighbours_with_directions()
            .into_iter()
            // the next tile cannot be a wall
            .filter(|(n, _)| match board.get_tile(*n) {
                Some(t) => *t != Wall,
                None => false
            })
            .map(move |(n, d)| if d == *dir {
                // the reindeer continues in the direction it is already heading, so
                // the cost of doing so is 1
                ((n, d), 1)
            } else {
                // the reindeer stays on its position, but it needs to turn
                // this costs 1000 points
                ((*pos, d), 1000)
            })
            .collect::<Vec<_>>(),
        // use manhattan distance as heuristic
        |(pos, _)| pos.manhattan_distance(&goal) as usize,
        |(pos, _)| pos == &goal
    ).unwrap();

    cost
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<Tile>::from_text(input);
    let start = (board.get_position_of(&Start).unwrap(), XP);
    let goal = board.get_position_of(&End).unwrap();

    // same as A, but I now collect all shortest paths using the astar_bag
    // what a cool crate!
    let (paths, _) = astar_bag_collect(
        &start,
        |(pos, dir)| pos.cardinal_neighbours_with_directions()
            .into_iter()
            .filter(|(n, _)| match board.get_tile(*n) {
                Some(t) => *t != Wall,
                None => false
            })
            .map(|(n, d)| if d == *dir {
                ((n, d), 1)
            } else {
                ((*pos, d), 1000)
            })
            .collect::<Vec<_>>(),
        |(pos, _)| pos.manhattan_distance(&goal) as usize,
        |(pos, _)| pos == &goal
    ).unwrap();

    let mut best_tiles = HashSet::new();
    best_tiles.extend(paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|(p, _)| p))
    );

    best_tiles.len()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Wall,
    Free
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Start,
            'E' => End,
            '#' => Wall,
            '.' => Free,
            _ => unreachable!()
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Start => 'S',
            End => 'E',
            Wall => '#',
            Free => '.'
        }
    }
}