use Tile::*;
use crate::aoc_lib::*;
use pathfinding::prelude::astar;
use itertools::Itertools;
use rayon::prelude::*;

pub fn solve_a(input: &str) -> usize {
    // I added the target threshold to the puzzle input
    let threshold = parse(input.lines().next().unwrap());
    let board_input = input.lines().skip(1).map(|line|line).join("\n");
    let board = Board::<Tile>::from(board_input.as_str());

    let start = board.get_positions_of(&Start).into_iter().next().unwrap();
    let goal = board.get_positions_of(&End).into_iter().next().unwrap();

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

    count_cheats(
        len,
        &path,
        2,
        threshold,
    )
}

pub fn solve_b(input: &str) -> usize {
    // I added the target threshold to the puzzle input
    let threshold = parse(input.lines().next().unwrap());
    let board_input = input.lines().skip(1).map(|line|line).join("\n");
    let board = Board::<Tile>::from(board_input.as_str());
    let start = board.get_positions_of(&Start).into_iter().next().unwrap();
    let goal = board.get_positions_of(&End).into_iter().next().unwrap();

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

    count_cheats(
        len,
        &path,
        20,
        threshold,
    )
}

fn count_cheats(
    shortest_path_len: usize,
    shortest_path: &Vec<Position>,
    max_cheat_len: usize,
    threshold: usize,
) -> usize {
    // General idea: There is only one path in the labyrinth, so it is not possible
    // to cheat your way to some position on another path to the goal. This means I
    // can only go from some position i on the shortest path to some position j on the
    // shortest path, where i and j are the indexes on the shortest path, i < j and
    // shortest_path[i] has a manhattan distance of the cheat length to shortest_path[j].
    // I therefore iterate over all positions in the path, look ahead to all positions after
    // it, only take the ones with a manhattan distance of my cheat distance into consideration and check if
    // the difference between my new path and the shortest path is smaller than the threshold.
    // Every match will be counted and the sum gets returned.

    // go from the min cheat length (always 2) to the max cheat length provided by the caller
    (2..=max_cheat_len)
        .into_iter()
        // rayon for speed
        .par_bridge()
        // iterate over the whole shortest path
        .map(|cheat| shortest_path
            .iter()
            .enumerate()
            // look ahead from the current position
            .flat_map(|(i, pos)| ((i + 1)..shortest_path.len())
                .into_iter()
                .map(|j| (j, shortest_path[j]))
                // only consider positions which have a manhattan distance of cheat
                .filter(move |(_, path_pos)| (pos.manhattan_distance(path_pos) as usize) == cheat)
                // Calculate the difference between the original path length and the resulting
                // length when using the cheat. Note: If the cheat would go along the original path,
                // which means going over 0 walls, this formula will return 0, so this case is handled too.
                .map(move |(j, _)| shortest_path_len - (i + cheat + (shortest_path_len - j)))
            )
            // keep only differences on or above the threshold
            .filter(|diff| *diff >= threshold)
            .count()
        )
        .sum()
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