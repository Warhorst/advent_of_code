use std::collections::HashSet;
use itertools::Itertools;
use Tile::*;
use helpers::prelude::*;

/// Hint: I added the number of taken steps to my puzzle input (first line)
pub fn solve_a(input: &str) -> usize {
    let mut lines = input.lines();
    let num_steps = lines.next().unwrap().parse::<usize>().unwrap();
    let mut tile_map = TileMap::<Tile>::from(lines.join("\n").as_str());

    count_visited_tiles(num_steps, &mut tile_map)
}

/// No idea so far. Maybe find out a pattern and skip ahead
pub fn solve_b(_input: &str) -> usize {
    0
}

fn count_visited_tiles(num_steps: usize, tile_map: &mut TileMap<Tile>) -> usize {
    // println!("{tile_map}");

    for _ in 0..num_steps {
        let visited_tiles = tile_map
            .iter()
            .filter_map(|(pos, tile)| match tile {
                Start | Visited => Some(pos),
                _ => None
            })
            .collect::<HashSet<_>>();

        visited_tiles
            .into_iter()
            .for_each(|pos| {
                tile_map.set(pos, Plot);
                pos.cardinal_neighbours()
                    .into_iter()
                    .for_each(|n| if let Some(Plot) = tile_map.try_get(n) {
                        tile_map.set(n, Visited)
                    })
            });

        // println!("{tile_map}");
    }

    tile_map.iter().filter(|(_, tile)| *tile == Visited || *tile == Start).count()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Start,
    Plot,
    Rock,
    Visited,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Start,
            '.' => Plot,
            '#' => Rock,
            'O' => Visited,
            _ => panic!("invalid")
        }
    }
}
