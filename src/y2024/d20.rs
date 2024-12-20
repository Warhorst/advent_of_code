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

    // find the path with the added rule that the path can go through the gap
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
    // instead of a path, the cheat can be seen as a jump, with a min manhatten distance of 2 and
    // a max manhattan distance of 20

    // I can only make one jump per pathfind

    // every position in the path has a remaining length to goal
    // I want to jump to positions where len - (jump_remaining_distance + taken_distance) >= 100

    // Speculation: I can only jump from one position in the best path to another position in the
    // best path -> therefore, no new pathfinding are necessary

    // a cheat starts when I walk into a wall

    let board = Board::<Tile>::from_text(input);
    let start = board.get_position_of(&Start).unwrap();
    let goal = board.get_position_of(&End).unwrap();

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

    println!("{len}");
    PositionPrinter::new().draw_axis(false).y_is_top(true).print(path);

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