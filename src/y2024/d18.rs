use crate::aoc_lib::*;
use Tile::*;
use pathfinding::prelude::astar;

pub fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();
    // I added the board dimension and the amount of bytes falling to my puzzle/example input
    let (dimension, fallen_bytes) = {
        let mut split = lines.next().unwrap().split(" ");
        (parse::<usize>(split.next().unwrap()), parse::<usize>(split.next().unwrap()))
    };
    // create a board with the given dimension (+1, as dimension 6 would go from 0 to 5)
    // with only free tiles
    let mut board = Board::<Tile>::from_width_height(dimension + 1, dimension + 1, Free);

    // add the bytes until the amount of fallen bytes is reached
    lines
        .take(fallen_bytes)
        .map(|line| {
            let mut split = line.split(",");
            p!(parse::<isize>(split.next().unwrap()), parse::<isize>(split.next().unwrap()))
        })
        .for_each(|pos| board.set_tile(pos, Corrupted));

    let start = p!(0, 0);
    let goal = p!(dimension, dimension);

    // pathfind
    let (_path, length) = astar(
        &start,
        |pos| pos
            .cardinal_neighbours()
            .into_iter()
            .filter(|n| match board.get_tile(*n) {
                Some(Free) => true,
                Some(Corrupted) => false,
                None => false
            })
            .map(|pos| (pos, 1)),
        |pos| pos.manhattan_distance(&goal) as usize,
        |pos| *pos == goal
    ).unwrap();

    length
}

pub fn solve_b(input: &str) -> String {
    let mut lines = input.lines();
    let (dimension, fallen_bytes) = {
        let mut split = lines.next().unwrap().split(" ");
        (parse::<usize>(split.next().unwrap()), parse::<usize>(split.next().unwrap()))
    };
    let mut board = Board::<Tile>::from_width_height(dimension + 1, dimension + 1, Free);

    let mut positions = lines
        .map(|line| {
            let mut split = line.split(",");
            p!(parse::<isize>(split.next().unwrap()), parse::<isize>(split.next().unwrap()))
        });

    let start = p!(0, 0);
    let goal = p!(dimension, dimension);
    let mut count = 0;

    while let Some(pos) = positions.next() {
        // add corrupted tiles in a loop
        board.set_tile(pos, Corrupted);
        count += 1;

        // small optimization: until fallen_bytes is exceeded, it is
        // not necessary to check if a path still exists, as
        // part A already showed that it does
        if count < fallen_bytes {
            continue
        }

        // check if a path still exists
        let path_opt = astar(
            &start,
            |pos| pos
                .cardinal_neighbours()
                .into_iter()
                .filter(|n| match board.get_tile(*n) {
                    Some(Free) => true,
                    Some(Corrupted) => false,
                    None => false
                })
                .map(|pos| (pos, 1)),
            |pos| pos.manhattan_distance(&goal) as usize,
            |pos| *pos == goal
        );

        if path_opt.is_none() {
            // if no path exists, return the position as string
            // which caused it
            return format!("{},{}", pos.x, pos.y)
        }
    }

    unreachable!()
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Free,
    Corrupted,
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Free => '.',
            Corrupted => '#'
        }
    }
}

//impl From<char> for Tile {
//    fn from(value: char) -> Self {
//        match value {
//            '.' => Free
//        }
//    }
//}