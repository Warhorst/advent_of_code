use std::collections::HashMap;
use geo::{Contains, Coord, LineString, point, Polygon};
use pad::{Direction, p, Position};
use pad::Direction::*;
use Tile::*;

pub fn solve_10a(input: &str) -> usize {
    let tile_map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (p!(j, i), Tile::from(c))))
        .collect::<HashMap<_, _>>();

    let start_pos = tile_map.iter().filter_map(|(pos, tile)| if tile == &Start {
        Some(*pos)
    } else {
        None
    }).next().unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let (mut current_pos, mut current_direction) = start_pos
        .cardinal_neighbours_with_directions()
        .into_iter()
        .filter(|(pos, _)| position_in_bounds(*pos, width, height))
        .filter(|(pos, dir)| tile_map.get(&pos).unwrap().next_direction(*dir).is_some())
        .next().unwrap();

    let mut count = 1;

    while tile_map.get(&current_pos).unwrap() != &Start {
        (current_pos, current_direction) = current_pos
            .cardinal_neighbours_with_directions()
            .into_iter()
            .filter(|(pos, _)| position_in_bounds(*pos, width, height))
            .filter(|(_, dir)| tile_map.get(&current_pos).unwrap().possible_directions().contains(dir))
            .filter(|(_, dir)| reverse(dir) != current_direction)
            .filter(|(pos, dir)| tile_map.get(&pos).unwrap().next_direction(*dir).is_some())
            .next().unwrap();

        count += 1
    }

    count / 2
}

pub fn solve_10b(input: &str) -> usize {
    let tile_map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (p!(j, i), Tile::from(c))))
        .collect::<HashMap<_, _>>();

    let start_pos = tile_map.iter().filter_map(|(pos, tile)| if tile == &Start {
        Some(*pos)
    } else {
        None
    }).next().unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut loop_positions = vec![];

    let (mut current_pos, mut current_direction) = start_pos
        .cardinal_neighbours_with_directions()
        .into_iter()
        .filter(|(pos, _)| position_in_bounds(*pos, width, height))
        .filter(|(pos, dir)| tile_map.get(&pos).unwrap().next_direction(*dir).is_some())
        .next().unwrap();

    loop_positions.push(current_pos);

    while tile_map.get(&current_pos).unwrap() != &Start {
        (current_pos, current_direction) = current_pos
            .cardinal_neighbours_with_directions()
            .into_iter()
            .filter(|(pos, _)| position_in_bounds(*pos, width, height))
            .filter(|(_, dir)| tile_map.get(&current_pos).unwrap().possible_directions().contains(dir))
            .filter(|(_, dir)| reverse(dir) != current_direction)
            .filter(|(pos, dir)| tile_map.get(&pos).unwrap().next_direction(*dir).is_some())
            .next().unwrap();

        loop_positions.push(current_pos);
    }

    let polygon = Polygon::new(LineString::new(
        loop_positions
            .iter()
            .map(|pos| {
                let mut c = Coord::zero();
                c.x = pos.x as f32;
                c.y = pos.y as f32;
                c
            })
            .collect()
    ),
                               vec![]);

    tile_map
        .keys()
        .filter(|pos| !loop_positions.contains(pos))
        .filter(|pos| polygon.contains(&point!(x: pos.x as f32, y: pos.y as f32)))
        .count()
}

fn position_in_bounds(pos: Position, width: usize, height: usize) -> bool {
    pos.x >= 0 && pos.x < width as isize && pos.y >= 0 && pos.y < height as isize
}

fn reverse(dir: &Direction) -> Direction {
    match *dir {
        XP => XM,
        XM => XP,
        YP => YM,
        YM => YP,
        _ => panic!("invalid dir")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn next_direction(&self, goiing: Direction) -> Option<Direction> {
        match self {
            Vertical => match goiing {
                YP => Some(YP),
                YM => Some(YM),
                _ => None
            }
            Horizontal => match goiing {
                XP => Some(XP),
                XM => Some(XM),
                _ => None
            }
            NorthEast => match goiing {
                YP => Some(XP),
                XM => Some(YM),
                _ => None
            }
            NorthWest => match goiing {
                XP => Some(YM),
                YP => Some(XM),
                _ => None
            }
            SouthWest => match goiing {
                YM => Some(XM),
                XP => Some(YP),
                _ => None
            }
            SouthEast => match goiing {
                YM => Some(XP),
                XM => Some(YP),
                _ => None
            }
            Ground => None,
            Start => Some(XP),
        }
    }

    fn possible_directions(&self) -> Vec<Direction> {
        match self {
            Vertical => vec![YP, YM],
            Horizontal => vec![XP, XM],
            NorthEast => vec![YM, XP],
            NorthWest => vec![YM, XM],
            SouthWest => vec![YP, XM],
            SouthEast => vec![YP, XP],
            Ground => vec![],
            Start => vec![XP, XM, YP, YM]
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            c => panic!("unknown char {c}")
        }
    }
}