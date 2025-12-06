use std::collections::HashMap;
use colored::Colorize;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);

    let start = (p!(0, 0), Vec::with_capacity(3));
    let goal = p!(tile_map.width - 1, tile_map.height - 1);

    let res = astar(
        &start,
        |n| successors(&tile_map, n.clone()),
        |(pos, _)| distance(*pos, goal),
        |(pos, _)| *pos == goal,
    );

    if let Some((_, c)) = res {
        c
    } else {
        0
    }
}

// TODO this does not work. I added 1 to the returned puzzle result, just for fun, and it was correct.
//  It works on the example input tho...
pub fn solve_b(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);

    let start = (p!(0, 0), Vec::with_capacity(10));
    let goal = p!(tile_map.width - 1, tile_map.height - 1);

    let res = astar(
        &start,
        |n| successors_ultra(&tile_map, n.clone()),
        |(pos, _)| distance(*pos, goal),
        |(pos, _)| *pos == goal,
    );

    if let Some((path, c)) = res {
        print_path(&tile_map, path);
        c
    } else {
        0
    }
}

fn successors<'a>(
    tilemap: &'a TileMap<Tile>,
    (pos, prev_dirs): (Position, Vec<Direction>),
) -> impl IntoIterator<Item=((Position, Vec<Direction>), usize)> + 'a {
    [XP, XM, YP, YM]
        .into_iter()
        .filter_map(move |dir| {
            if prev_dirs.last() == Some(&dir.opposite()) {
                return None;
            }

            if prev_dirs.len() == 3 && prev_dirs.iter().all(|pd| *pd == dir) {
                return None;
            }

            let pos_in_dir = pos.position_in_direction(dir, 1);
            let mut new_prevs = prev_dirs.clone();

            match prev_dirs.len() {
                3 => {
                    new_prevs.remove(0);
                    new_prevs.push(dir)
                }
                _ => new_prevs.push(dir)
            };

            tilemap.try_get(pos_in_dir).map(|t| ((pos_in_dir, new_prevs), t.0))
        })
}

fn successors_ultra<'a>(
    tilemap: &'a TileMap<Tile>,
    (pos, prev_dirs): (Position, Vec<Direction>),
) -> impl IntoIterator<Item=((Position, Vec<Direction>), usize)> + 'a {
    [XP, XM, YP, YM]
        .into_iter()
        .filter_map(move |dir| {
            if prev_dirs.last() == Some(&dir.opposite()) {
                return None;
            }

            if let Some(w) = prev_dirs.windows(4).last() {
                if w.iter().any(|wd| *wd != w[3]) && w[3] != dir {
                    return None
                }
            } else if let Some(d) = prev_dirs.last() && *d != dir {
                return None
            }

            if prev_dirs.len() == 10 && prev_dirs.iter().all(|pd| *pd == dir) {
                return None;
            }

            let pos_in_dir = pos.position_in_direction(dir, 1);
            let mut new_prevs = prev_dirs.clone();

            match prev_dirs.len() {
                10 => {
                    new_prevs.remove(0);
                    new_prevs.push(dir)
                }
                _ => new_prevs.push(dir)
            };

            tilemap.try_get(pos_in_dir).map(|t| ((pos_in_dir, new_prevs), t.0))
        })
}

fn distance(pos: Position, goal: Position) -> usize {
    (pos.x - goal.x).unsigned_abs() + (pos.y - goal.y).unsigned_abs()
}

/// For debugging
#[allow(dead_code)]
fn print_path(
    tile_map: &TileMap<Tile>,
    path: Vec<(Position, Vec<Direction>)>,
) {
    let map = path
        .into_iter()
        .collect::<HashMap<_, _>>();

    for y in 0..tile_map.height {
        for x in 0..tile_map.width {
            let pos = p!(x, y);

            if let Some(dirs) = map.get(&pos) {
                match dirs.last() {
                    Some(XP) => print!("{}", ">".red()),
                    Some(XM) => print!("{}", "<".red()),
                    Some(YP) => print!("{}", "v".red()),
                    Some(YM) => print!("{}", "^".red()),
                    _ => print!("{}", "s".red()) // the only tile with no direction is start
                }
            } else {
                print!("{}", <Tile as Into<char>>::into(tile_map.get(pos)))
            }
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

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        char::from_digit(value.0 as u32, 10).unwrap()
    }
}
