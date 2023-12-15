use pad::p;
use Tile::*;
use crate::aoc_lib::*;

pub fn solve_14a(input: &str) -> usize {
    let mut tile_map = TileMap::<Tile>::from(input);

    slide_north(&mut tile_map);

    calculate_north_weight(&tile_map)
}

pub fn solve_14b(input: &str) -> usize {
    let mut tile_map = TileMap::<Tile>::from(input);
    run_n_times_with_cycle(
        1_000_000_000,
        &mut tile_map,
        |tm| {
            slide_north(tm);
            slide_west(tm);
            slide_south(tm);
            slide_east(tm);
        }
    );

    calculate_north_weight(&tile_map)
}

fn slide_north(tile_map: &mut TileMap<Tile>) {
    for y in 1..tile_map.height {
        for x in 0..tile_map.width {
            let mut pos = p!(x, y);

            if let Some(Empty) | Some(Cube) = tile_map.try_get(pos) {
                continue
            }

            while let Some(Empty) = tile_map.try_get(p!(pos.x, pos.y - 1)) {
                tile_map.set(pos, Empty);
                tile_map.set(p!(pos.x, pos.y - 1), Round);
                pos = p!(pos.x, pos.y - 1);
            }
        }
    }
}

fn slide_west(tile_map: &mut TileMap<Tile>) {
    for x in 1..tile_map.width {
        for y in 0..tile_map.height {
            let mut pos = p!(x, y);

            if let Some(Empty) | Some(Cube) = tile_map.try_get(pos) {
                continue
            }

            while let Some(Empty) = tile_map.try_get(p!(pos.x - 1, pos.y)) {
                tile_map.set(pos, Empty);
                tile_map.set(p!(pos.x - 1, pos.y), Round);
                pos = p!(pos.x - 1, pos.y);
            }
        }
    }
}

fn slide_south(tile_map: &mut TileMap<Tile>) {
    for y in (0..tile_map.height - 1).rev() {
        for x in 0..tile_map.width {
            let mut pos = p!(x, y);

            if let Some(Empty) | Some(Cube) = tile_map.try_get(pos) {
                continue
            }

            while let Some(Empty) = tile_map.try_get(p!(pos.x, pos.y + 1)) {
                tile_map.set(pos, Empty);
                tile_map.set(p!(pos.x, pos.y + 1), Round);
                pos = p!(pos.x, pos.y + 1);
            }
        }
    }
}

fn slide_east(tile_map: &mut TileMap<Tile>) {
    for x in (0..tile_map.width - 1).rev() {
        for y in 0..tile_map.height {
            let mut pos = p!(x, y);

            if let Some(Empty) | Some(Cube) = tile_map.try_get(pos) {
                continue
            }

            while let Some(Empty) = tile_map.try_get(p!(pos.x + 1, pos.y)) {
                tile_map.set(pos, Empty);
                tile_map.set(p!(pos.x + 1, pos.y), Round);
                pos = p!(pos.x + 1, pos.y);
            }
        }
    }
}

fn calculate_north_weight(tile_map: &TileMap<Tile>) -> usize {
    tile_map
        .rows()
        .into_iter()
        .enumerate()
        .map(|(i, row)| (tile_map.height - i) * row.iter().filter(|t| t == &&Round).count())
        .sum()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Round,
    Cube,
    Empty
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Round,
            '#' => Cube,
            '.' => Empty,
            _ => panic!("invalid char")
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Round => 'O',
            Cube => '#',
            Empty => '.'
        }
    }
}