use itertools::Itertools;
use pad::{p, Position};
use pathfinding::prelude::astar;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use Tile::*;

pub fn solve_a(input: &str) -> usize {
    let mut space = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();


    expand_space(&mut space);

    // space.iter().for_each(|line| println!("{:?}", line));

    let galaxy_positions = space
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line
            .iter()
            .enumerate()
            .filter_map(move |(x, tile)| match tile {
                Galaxy => Some(p!(x, y)),
                Nothing => None
            })
        ).collect::<Vec<_>>();

    let width = space[0].len();
    let height = space.len();

    let pairs = galaxy_positions.iter().copied().combinations(2).collect::<Vec<_>>();

    println!("{}", pairs.len());

    pairs
        .into_par_iter()
        .map(|pair| {
            let start = pair[0];
            let goal = pair[1];
            astar(
                &start,
                |pos: &Position| pos.cardinal_neighbours()
                    .into_iter()
                    .filter(|n| n.x >= 0 && n.x < width as isize && n.y >= 0 && n.y < height as isize)
                    .map(|n| (n, 1)),
                |pos| distance(pos, &goal),
                |pos| pos == &goal,
            ).unwrap().1 as usize
        })
        .sum::<usize>()
}

pub fn solve_b(input: &str) -> u128 {
    let space = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (empty_rows, empty_columns) = find_empty_rows_and_columns(&space);

    let pairs = space
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line
            .iter()
            .enumerate()
            .filter_map(move |(x, tile)| match tile {
                Galaxy => Some(p!(x, y)),
                Nothing => None
            })
        )
        .combinations(2)
        .collect::<Vec<_>>();

    pairs
        .into_par_iter()
        .map(|pair| {
            let start = pair[0];
            let goal = pair[1];

            let (x_diff, y_diff) = ((start.x - goal.x).abs() as usize, (start.y - goal.y).abs() as usize);

            let empty_rows_between = if start.y < goal.y {
                start.y..goal.y
            } else {
                goal.y..start.y
            }.filter(|i| empty_rows.contains(&(*i as usize))).count();

            let empty_columns_between = if start.x < goal.x {
                start.x..goal.x
            } else {
                goal.x..start.x
            }.filter(|i| empty_columns.contains(&(*i as usize))).count();

            ((x_diff - empty_columns_between) + empty_columns_between * 1_000_000 + (y_diff - empty_rows_between) + empty_rows_between * 1_000_000) as u128
        })
        .sum::<u128>()
}

fn expand_space(space: &mut Vec<Vec<Tile>>) {
    let width = space[0].len();
    let height = space.len();

    for i in 0..height {
        if space[height - 1 - i].iter().all(|t| *t == Nothing) {
            space.insert(height - i, vec![Nothing; width])
        }
    }

    for i in 0..width {
        if space.iter().all(|line| line[width - 1 - i] == Nothing) {
            space.iter_mut().for_each(|line| line.insert(width - i, Nothing))
        }
    }
}

fn find_empty_rows_and_columns(space: &Vec<Vec<Tile>>) -> (Vec<usize>, Vec<usize>) {
    let width = space[0].len();
    let height = space.len();

    let mut rows = vec![];
    let mut columns = vec![];

    for i in 0..width {
        if space.iter().all(|line| line[i] == Nothing) {
            columns.push(i)
        }
    }

    for i in 0..height {
        if space[i].iter().all(|t| *t == Nothing) {
            rows.push(i)
        }
    }

    (rows, columns)
}

fn distance(pos_a: &Position, pos_b: &Position) -> isize {
    (pos_a.x - pos_b.x).abs() + (pos_a.y - pos_b.y).abs()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Galaxy,
    Nothing,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Galaxy,
            '.' => Nothing,
            _ => panic!("invalid char")
        }
    }
}