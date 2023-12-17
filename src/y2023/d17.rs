use std::collections::HashMap;
use colored::Colorize;
use crate::aoc_lib::*;

pub fn solve_17a(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);

    println!("{tile_map}");

    let start = (p!(0, 0), None);
    let goal = p!(tile_map.width - 1, tile_map.height - 1);

    let res = astar(
        &start,
        |n| successors(&tile_map, *n), // successors
        |(pos, _)| distance(*pos, goal), // heuristic
        |(pos, _)| *pos == goal
    );

    if let Some((path, c)) = res {
        print_path(&tile_map, path);
        c
    } else {
        0
    }
}

pub fn solve_17b(input: &str) -> usize {
    0
}

fn successors(
    tilemap: &TileMap<Tile>,
    (pos, dir_opt): (Position, Option<Direction>)
) -> impl IntoIterator<Item=((Position, Option<Direction>), usize)> + '_ {
    [XP, XM, YP, YM]
        .into_iter()
        .filter(move |dir| match dir_opt {
            Some(d) => dir.opposite() != d,
            None => true
        })
        .flat_map(move |dir| {
            let mut v = Vec::with_capacity(3);
            let mut cost = 0;

            for i in 1..=3 {
                let pos_in_dir = pos.position_in_direction(dir, i);

                if let Some(t) = tilemap.try_get(pos_in_dir) {
                    cost += t.0;
                    v.push(((pos_in_dir, Some(dir)), cost));
                } else {
                    break
                }
            }

            v
        })
}

fn distance(pos: Position, goal: Position) -> usize {
    (pos.x - goal.x).abs() as usize + (pos.y - goal.y).abs() as usize
}

/// For debugging
fn print_path(
    tile_map: &TileMap<Tile>,
    path: Vec<(Position, Option<Direction>)>
) {
    let map = path
        .into_iter()
        .collect::<HashMap<_, _>>();

    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            let pos = p!(x, y);

            if let Some(dir) = map.get(&pos) {
                match dir {
                    Some(XP) => print!("{}", ">".red()),
                    Some(XM) => print!("{}", "<".red()),
                    Some(YP) => print!("{}", "v".red()),
                    Some(YM) => print!("{}", "^".red()),
                    _ => print!("{}", "s".red()) // the only tile with no direction is start
                }
            } else {
                print!("{}", <Tile as Into<char>>::into(tile_map.get(pos)))
            }
            print!(" ")
        }
        println!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Tile(usize);

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile(value.to_digit(10).unwrap() as usize)
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        char::from_digit(self.0 as u32, 10).unwrap()
    }
}