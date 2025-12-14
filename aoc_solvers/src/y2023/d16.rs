use std::collections::HashSet;
use helpers::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::y2023::d16::Tile::*;

pub fn solve_a(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);

    let mut beamer = Beamer::new(&tile_map, p!(0, 0), XP);
    beamer.beam();
    beamer.count_unique_energized_tiles()
}

pub fn solve_b(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);

    let positions_and_directions = (0..tile_map.width)
        .flat_map(|x| [(p!(x, 0), YP), (p!(x, tile_map.height - 1), YM)])
        .chain((0..tile_map.height)
            .flat_map(|y| [(p!(0, y), XP), (p!(tile_map.width - 1, y), XM)])
        )
        .collect::<Vec<_>>();

    positions_and_directions
        .into_par_iter()
        .map(|(pos, dir)| {
            let mut beamer = Beamer::new(&tile_map, pos, dir);
            beamer.beam();
            beamer.count_unique_energized_tiles()
        })
        .max().unwrap()
}

#[derive(Clone, Eq, PartialEq)]
struct Beamer<'a> {
    beams: HashSet<(Position, Direction)>,
    tile_map: &'a TileMap<Tile>,
}

impl<'a> Beamer<'a> {
    fn new(tile_map: &'a TileMap<Tile>, start_pos: Position, start_dir: Direction) -> Self {
        let mut beams = HashSet::new();

        match start_dir {
            XP => match tile_map.get(start_pos) {
                Empty => {
                    beams.insert((start_pos, XP));
                }
                RMirror => {
                    beams.insert((start_pos, YM));
                }
                LMirror => {
                    beams.insert((start_pos, YP));
                }
                HSplitter => {
                    beams.insert((start_pos, XP));
                }
                VSplitter => {
                    beams.insert((start_pos, YP));
                    beams.insert((start_pos, YM));
                }
            }
            YP => match tile_map.get(start_pos) {
                Empty => {
                    beams.insert((start_pos, YP));
                }
                RMirror => {
                    beams.insert((start_pos, XM));
                }
                LMirror => {
                    beams.insert((start_pos, XP));
                }
                HSplitter => {
                    beams.insert((start_pos, XP));
                    beams.insert((start_pos, XM));
                }
                VSplitter => {
                    beams.insert((start_pos, YP));
                }
            }
            XM => match tile_map.get(start_pos) {
                Empty => {
                    beams.insert((start_pos, XM));
                }
                RMirror => {
                    beams.insert((start_pos, YP));
                }
                LMirror => {
                    beams.insert((start_pos, YM));
                }
                HSplitter => {
                    beams.insert((start_pos, XM));
                }
                VSplitter => {
                    beams.insert((start_pos, YP));
                    beams.insert((start_pos, YM));
                }
            }
            YM => match tile_map.get(start_pos) {
                Empty => {
                    beams.insert((start_pos, YM));
                }
                RMirror => {
                    beams.insert((start_pos, XP));
                }
                LMirror => {
                    beams.insert((start_pos, XM));
                }
                HSplitter => {
                    beams.insert((start_pos, XP));
                    beams.insert((start_pos, XM));
                }
                VSplitter => {
                    beams.insert((start_pos, YM));
                }
            }
            _ => {}
        }

        Beamer {
            beams,
            tile_map,
        }
    }

    fn beam(&mut self) {
        loop {
            let prev = self.clone();
            self.update();

            if *self == prev {
                return;
            }
        }
    }

    fn update(&mut self) {
        let mut new_state = HashSet::new();
        let mut insert = |pos: Position, dir: Direction| {
            if self.tile_map.pos_in_bounds(pos) {
                new_state.insert((pos, dir));
            }
        };

        self.beams
            .iter()
            .for_each(|(pos, dir)| {
                let pos_in_dir = pos.position_in_direction(*dir, 1);

                insert(*pos, *dir);

                match self.tile_map.try_get(pos_in_dir) {
                    Some(Empty) => insert(pos_in_dir, *dir),
                    Some(RMirror) => match *dir {
                        XP => {
                            insert(pos_in_dir, YM);
                        }
                        XM => {
                            insert(pos_in_dir, YP);
                        }
                        YP => {
                            insert(pos_in_dir, XM);
                        }
                        YM => {
                            insert(pos_in_dir, XP);
                        }
                        _ => {}
                    }
                    Some(LMirror) => match *dir {
                        XP => {
                            insert(pos_in_dir, YP);
                        }
                        XM => {
                            insert(pos_in_dir, YM);
                        }
                        YP => {
                            insert(pos_in_dir, XP);
                        }
                        YM => {
                            insert(pos_in_dir, XM);
                        }
                        _ => {}
                    }
                    Some(HSplitter) => match *dir {
                        XP => {
                            insert(pos_in_dir, XP);
                        }
                        XM => {
                            insert(pos_in_dir, XM);
                        }
                        YP => {
                            insert(pos_in_dir, XP);
                            insert(pos_in_dir, XM);
                        }
                        YM => {
                            insert(pos_in_dir, XP);
                            insert(pos_in_dir, XM);
                        }
                        _ => {}
                    }
                    Some(VSplitter) => match *dir {
                        XP => {
                            insert(pos_in_dir, YM);
                            insert(pos_in_dir, YP);
                        }
                        XM => {
                            insert(pos_in_dir, YM);
                            insert(pos_in_dir, YP);
                        }
                        YP => {
                            insert(pos_in_dir, YP);
                        }
                        YM => {
                            insert(pos_in_dir, YM);
                        }
                        _ => {}
                    },
                    None => {}
                }
            });

        self.beams = new_state;
    }

    fn count_unique_energized_tiles(&self) -> usize {
        let mut energized_positions = HashSet::new();
        self.beams.iter().for_each(|(pos, _)| {
            energized_positions.insert(*pos);
        });
        energized_positions.len()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    RMirror,
    LMirror,
    HSplitter,
    VSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '/' => RMirror,
            '\\' => LMirror,
            '-' => HSplitter,
            '|' => VSplitter,
            _ => panic!("invalid char")
        }
    }
}
