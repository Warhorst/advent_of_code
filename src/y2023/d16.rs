use std::collections::{HashMap, HashSet};
use pad::{Direction, p, Position};
use pad::Direction::*;
use crate::aoc_lib::*;
use crate::y2023::d16::Tile::*;

pub fn solve_16a(input: &str) -> usize {
    let tile_map = TileMap::<Tile>::from(input);
    println!("{tile_map}");

    let mut beamer = Beamer::new(tile_map);
    beamer.beam();
    beamer.print_energized_tiles();
    beamer.count_unique_energized_tiles()
}

pub fn solve_16b(input: &str) -> usize {
    0
}

#[derive(Clone, Eq, PartialEq)]
struct Beamer {
    /// (position, dir of beam) to is_front_beam
    beams: HashMap<(Position, Direction), bool>,
    tile_map: TileMap<Tile>,
}

impl Beamer {
    fn new(tile_map: TileMap<Tile>) -> Self {
        let mut beams = HashMap::new();

        match tile_map.get(p!(0, 0)) {
            Empty => {
                beams.insert((p!(0, 0), XP), true);
            }
            RMirror => {
                beams.insert((p!(0, 0), YM), true);
            }
            LMirror => {
                beams.insert((p!(0, 0), YP), true);
            }
            HSplitter => {
                beams.insert((p!(0, 0), XP), true);
            }
            VSplitter => {
                beams.insert((p!(0, 0), YP), true);
            }
        }

        beams.insert((p!(0, 0), XP), true);

        Beamer {
            beams,
            tile_map,
        }
    }

    fn beam(&mut self) {
        loop {
            let prev = self.count_energized_tiles();
            self.update();

            if self.count_energized_tiles() == prev {
                return;
            }
        }
    }

    fn update(&mut self) {
        let mut new_state = HashMap::new();
        let mut insert = |pos: Position, dir: Direction, is_front_beam: bool| {
            if self.tile_map.pos_in_bounds(pos) {
                new_state.entry((pos, dir)).and_modify(|front_beam: &mut bool| *front_beam = is_front_beam).or_insert(is_front_beam);
            }
        };

        self.beams
            .iter()
            .for_each(|((pos, dir), _)| {
                let pos_in_dir = pos.position_in_direction(*dir, 1);

                insert(*pos, *dir, false);

                match self.tile_map.try_get(pos_in_dir) {
                    Some(Empty) => insert(pos_in_dir, *dir, true),
                    Some(RMirror) => match *dir {
                        XP => {
                            insert(pos_in_dir, YM, true);
                        }
                        XM => {
                            insert(pos_in_dir, YP, true);
                        }
                        YP => {
                            insert(pos_in_dir, XM, true);
                        }
                        YM => {
                            insert(pos_in_dir, XP, true);
                        }
                        _ => {}
                    }
                    Some(LMirror) => match *dir {
                        XP => {
                            insert(pos_in_dir, YP, true);
                        }
                        XM => {
                            insert(pos_in_dir, YM, true);
                        }
                        YP => {
                            insert(pos_in_dir, XP, true);
                        }
                        YM => {
                            insert(pos_in_dir, XM, true);
                        }
                        _ => {}
                    }
                    Some(HSplitter) => match *dir {
                        XP => {
                            insert(pos_in_dir, XP, true);
                        }
                        XM => {
                            insert(pos_in_dir, XM, true);
                        }
                        YP => {
                            insert(pos_in_dir, XP, true);
                            insert(pos_in_dir, XM, true);
                        }
                        YM => {
                            insert(pos_in_dir, XP, true);
                            insert(pos_in_dir, XM, true);
                        }
                        _ => {}
                    }
                    Some(VSplitter) => match *dir {
                        XP => {
                            insert(pos_in_dir, YM, true);
                            insert(pos_in_dir, YP, true);
                        }
                        XM => {
                            insert(pos_in_dir, YM, true);
                            insert(pos_in_dir, YP, true);
                        }
                        YP => {
                            insert(pos_in_dir, YP, true);
                        }
                        YM => {
                            insert(pos_in_dir, YM, true);
                        }
                        _ => {}
                    },
                    None => {}
                }
            });

        self.beams = new_state;
    }

    fn print_energized_tiles(&self) {
        for y in 0..self.tile_map.height {
            for x in 0..self.tile_map.width {
                let beams_on_pos = self.beams.keys().filter(|(pos, _)| *pos == p!(x, y)).count();

                if beams_on_pos == 1 {
                    print!("#");
                } else if beams_on_pos > 1 {
                    print!("{beams_on_pos}");
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    fn count_energized_tiles(&self) -> usize {
        self.beams.keys().count()
    }

    fn count_unique_energized_tiles(&self) -> usize {
        let mut energized_positions = HashSet::new();
        self.beams.keys().for_each(|(pos, _)| {
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

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Empty => '.',
            RMirror => '/',
            LMirror => '\\',
            HSplitter => '-',
            VSplitter => '|'
        }
    }
}