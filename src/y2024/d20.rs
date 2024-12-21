use std::collections::{HashMap, HashSet};
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

    println!("Len: {len}");
    //println!("{:?}", path);

    let mut stats = HashMap::<usize, usize>::new();

    // idea
    //.filter_map(move |(j, _)| match (i..i + j).into_iter().any(|index| board.get_tile(path[index]).unwrap() == &Wall) {
    //    true => Some(j - 1),
    //    false => None
    //})

    let res = path
        .iter()
        .enumerate()
        .map(|(i, pos)| pos
            .cardinal_neighbours()
            .into_iter()
            .filter(|n| board.get_tile(*n).unwrap() == &Wall)
            .flat_map(|n| path[(i + 1)..]
                .into_iter()
                .enumerate()
                .filter(move |(_, path_pos)| (n.manhattan_distance(path_pos) as usize) == 2 - 1)
                .filter(|(j, _)| *j > 0)
                .map(|(j, _)| j - 1)
            )
            .inspect(|diff| { stats.entry(*diff).and_modify(|val| *val += 1).or_insert(1); })
            .filter(|diff| *diff >= 100)
            .count()
        )
        .sum();

    //let res = path
    //    .iter()
    //    .enumerate()
    //    .flat_map(|(i, pos)| path[(i + 1)..]
    //        .into_iter()
    //        .enumerate()
    //        .map(move |(j, path_pos)| ((i, pos), (j, path_pos)))
    //    )
    //    .filter(|((i, pos), (_, path_pos))| (pos.manhattan_distance(path_pos) as usize) == 2)
    //    .filter_map(|((i, pos), (j, _))| match (i..i + j).into_iter().any(|index| board.get_tile(path[index]).unwrap() == &Wall) {
    //        true => Some(j - 1),
    //        false => None
    //    })
    //    .inspect(|diff| { stats.entry(*diff).and_modify(|val| *val += 1).or_insert(1); })
    //    .filter(|diff| *diff >= 100)
    //    .count();

    let mut stats = stats
        .into_iter()
        .collect::<Vec<_>>();
    stats.sort();
    stats.into_iter().for_each(|(key, value)| println!("{key} : {value}"));

    res
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

    let (path, _len) = astar(
        &start,
        |pos| pos
            .cardinal_neighbours()
            .into_iter()
            .filter(|n| *board.get_tile(*n).unwrap() != Wall)
            .map(|n| (n, 1)),
        |pos| pos.manhattan_distance(&goal) as usize,
        |pos| *pos == goal,
    ).unwrap();

    let mut stats = HashMap::<usize, usize>::new();

    let mut res = 0;

    let mut foo = HashMap::<usize, HashSet<(Position, Position)>>::new();

    for cheat in 2..=20 {
        res += path
            .iter()
            .enumerate()
            .map(|(i, pos)| pos
                .cardinal_neighbours()
                .into_iter()
                .filter(|n| board.get_tile(*n).unwrap() == &Wall)
                .flat_map(|n| path[(i + 1)..]
                    .into_iter()
                    .enumerate()
                    .filter(move |(_, path_pos)| (n.manhattan_distance(path_pos) as usize) == cheat - 1)
                    .filter(|(j, _)| *j > 0)
                    .map(|(j, path_pos)| (j - 1, path_pos))
                )
                .inspect(|(diff, _)| { stats.entry(*diff).and_modify(|val| *val += 1).or_insert(1); })
                .filter(|(diff, _)| *diff >= 50)
                .inspect(|(diff, path_pos)| {
                    foo.entry(*diff).and_modify(|set| { set.insert((*pos, **path_pos)); }).or_insert(HashSet::new());
                })
                .count()
            )
            .sum::<usize>();
    }

    let foo_len = foo.len();
    let mut foo = foo
        .into_iter()
        .collect::<Vec<_>>();
    foo.sort_by(|a, b| a.0.cmp(&b.0));
    foo.into_iter().for_each(|(key, set)| println!("{} : {}", set.len(), key));

    println!("Foo {}", foo_len);

    //let res = (2..=20)
    //    .into_iter()
    //    .map(|cheat| path
    //        .iter()
    //        .enumerate()
    //        .map(|(i, pos)| pos
    //            .cardinal_neighbours()
    //            .into_iter()
    //            .filter(|n| board.get_tile(*n).unwrap() == &Wall)
    //            .flat_map(|n| path[(i + 1)..]
    //                .into_iter()
    //                .enumerate()
    //                .filter(move |(_, path_pos)| (n.manhattan_distance(path_pos) as usize) == cheat - 1)
    //                .map(|(j, _)| j - 1)
    //            )
    //            .inspect(|diff| { stats.entry(*diff).and_modify(|val| *val += 1).or_insert(1); })
    //            .filter(|diff| *diff >= 100)
    //            .count()
    //        )
    //        .sum::<usize>())
    //    .sum();

    let mut stats = stats
        .into_iter()
        .collect::<Vec<_>>();
    stats.sort();
    //stats.into_iter().for_each(|(key, value)| println!("{key} : {value}"));

    res

    //path
    //    .iter()
    //    .enumerate()
    //    .map(|(i, pos)| pos
    //        .cardinal_neighbours()
    //        .into_iter()
    //        .filter(|n| board.get_tile(*n).unwrap() == &Wall)
    //        .flat_map(|n| path[(i + 1)..]
    //            .into_iter()
    //            .enumerate()
    //            .filter(move |(_, path_pos)| (n.manhattan_distance(path_pos) as usize) == 2 - 1)
    //            .map(|(j, _)| j)
    //        )
    //        .filter(|diff| *diff >= 100)
    //        .count())
    //    .sum();

    //0
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