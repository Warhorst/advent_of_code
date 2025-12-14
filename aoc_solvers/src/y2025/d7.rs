use std::collections::{HashMap, HashSet};

use proc_macros::tile;

use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    let mut board = Board::<Tile>::from(input);
    let mut y = 0;
    let mut split_count = 0;

    loop {
        // Sove part 1 by just simulating the split on the board.
        
        if y == board.rows().count() - 1 {
            break;
        }

        let beam_positions = board
            .line_to_border(p!(0, y), Direction::XP)
            .unwrap()
            .filter_map(|(pos, t)| if *t == Tile::Beam { Some(pos) } else { None })
            .collect::<Vec<_>>();
        let split_positions = board
            .line_to_border(p!(0, y + 1), Direction::XP)
            .unwrap()
            .filter_map(|(pos, t)| if *t == Tile::Split { Some(pos) } else { None })
            .collect::<Vec<_>>();

        for b_pos in beam_positions {
            let pos_below = p!(b_pos.x, y + 1);

            let split_opt = split_positions.iter().find(|p| **p == pos_below);

            match split_opt {
                Some(s_pos) => {
                    split_count += 1;
                    let _ = board.set_tile(p!(s_pos.x - 1, s_pos.y), Tile::Beam);
                    let _ = board.set_tile(p!(s_pos.x + 1, s_pos.y), Tile::Beam);
                }
                None => {
                    let _ = board.set_tile(p!(b_pos.x, y + 1), Tile::Beam);
                }
            }
        }

        y += 1;
    }

    split_count
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<Tile>::from(input);

    let beam = board
        .tiles_and_positions()
        .find_map(|(t, p)| if *t == Tile::Beam { Some(p) } else { None })
        .unwrap();
    let split_positions = board
        .tiles_and_positions()
        .filter_map(|(t, p)| if *t == Tile::Split { Some(p) } else { None })
        .collect::<HashSet<_>>();
    let bounds = Bounds {
        min_x: 0,
        min_y: 0,
        max_x: board.columns().count() as isize - 1,
        max_y: board.rows().count() as isize - 1,
    };
    let mut cache = HashMap::new();

    count_possibilities(beam, &split_positions, bounds, &mut cache)
}

fn count_possibilities(
    beam: Position,
    split_positions: &HashSet<Position>,
    bounds: Bounds,
    cache: &mut HashMap<Position, usize>,
) -> usize {
    if beam.y == bounds.max_y {
        // The end was reached, so this is one possiblity
        return 1;
    }

    if !bounds.contains_position(beam) {
        // The beam is out of bounds, so ignore it
        return 0;
    }

    if let Some(val) = cache.get(&beam) {
        // If the value for this beam was already determined, return it
        return *val;
    }

    let below = p!(beam.x, beam.y + 1);

    let res = if split_positions.contains(&below) {
        // Hits split
        let left = p!(beam.x - 1, beam.y + 1);
        let right = p!(beam.x + 1, beam.y + 1);

        count_possibilities(left, split_positions, bounds, cache)
            + count_possibilities(right, split_positions, bounds, cache)
    } else {
        // Hits nothing
        count_possibilities(below, split_positions, bounds, cache)
    };

    // Update the cache
    cache.insert(beam, res);

    res
}

#[tile]
enum Tile {
    #[t('S')]
    Beam,
    #[t('^')]
    Split,
    #[t('.')]
    Empty,
}
