use std::collections::HashSet;
use Tile::*;
use crate::aoc_lib::*;
use pathfinding::prelude::astar;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from_text(input);

    let start = board.get_position_of(&Start).unwrap();
    let goal = board.get_position_of(&End).unwrap();

    // get the best path and its len when using no cheats
    let (path, len) = astar(
        &start,
        |pos| pos
            .cardinal_neighbours()
            .into_iter()
            .filter(|n| *board.get_tile(*n).unwrap() != Wall)
            .map(|n| (n, 1)),
        |pos| pos.manhattan_distance(&goal) as usize,
        |pos| *pos == goal,
    ).unwrap();

    // find every wall that is between 2 positions of the best path
    let potential_gaps = path
        .iter()
        .flat_map(|pos| pos.cardinal_neighbours())
        .filter(|pos| *board.get_tile(*pos).unwrap() == Wall)
        .map(|wall| (wall, wall
            .cardinal_neighbours_with_directions()
            .into_iter()
            .filter(|(n, _)| match board.get_tile(*n) {
                Some(t) => *t != Wall,
                None => false
            })
            .collect::<Vec<_>>())
        )
        .filter(|(_, ns)| ns.len() == 2)
        .filter(|(_, ns)| match (ns[0].1, ns[1].1) {
            (XP, XM) | (XM, XP) | (YP, YM) | (YM, YP) => true,
            _ => false
        })
        .map(|(wall, _)| wall)
        .collect::<HashSet<_>>();

    // find the path with the added rule that the path can through the gap
    potential_gaps
        .into_iter()
        .flat_map(|gap| astar(
            &start,
            |pos| pos
                .cardinal_neighbours()
                .into_iter()
                .filter(|n| match board.get_tile(*n) {
                    Some(Wall) => *n == gap,
                    _ => true
                })
                .map(|n| (n, 1)),
            |pos| pos.manhattan_distance(&goal) as usize,
            |pos| *pos == goal,
        ))
        .map(|(_, new_len)| len - new_len)
        .filter(|len| *len >= 100)
        .count()
}

pub fn solve_b(input: &str) -> usize {
    0
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Wall,
    Free,
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