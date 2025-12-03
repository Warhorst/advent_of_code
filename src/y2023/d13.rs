use proc_macros::tile;

use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let blocks = input
        .split("\n\n")
        .map(TileMap::<Tile>::from)
        .collect::<Vec<_>>();

    blocks
        .into_iter()
        .map(get_value)
        .sum()
}

pub fn solve_b(_input: &str) -> usize {
    0
}

fn get_value(tile_map: TileMap<Tile>) -> usize {
    let columns = tile_map
        .columns()
        .into_iter()
        .collect::<Vec<_>>();

    let column_opt = (0..columns.len() - 1)
        .flat_map(|i| {
            let mut depth = 0;

            while i.checked_sub(depth).map(|_| true).unwrap_or(false)  && i + 1 + depth < tile_map.width  {
                if columns[i - depth] != columns[i + 1 + depth] {
                    return None
                }

                depth += 1
            }

            Some(i + 1)
        })
        .next();

    if let Some(val) = column_opt {
        return val
    }

    let rows = tile_map
        .rows()
        .into_iter()
        .collect::<Vec<_>>();

    (0..rows.len() - 1)
        .flat_map(|i| {
            let mut depth = 0;

            while i.checked_sub(depth).map(|_| true).unwrap_or(false) && i + 1 + depth < tile_map.height  {
                if rows[i - depth] != rows[i + 1 + depth] {
                    return None
                }

                depth += 1
            }

            Some(i + 1)
        })
        .next()
        .unwrap() * 100
}

#[tile]
enum Tile {
    #[t('.')]
    Ash,
    #[t('#')]
    Rock,
}
