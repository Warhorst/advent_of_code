use colored::Color::Black;
use geo::{Contains, Coord, LineString, point, Polygon};

use Tile::*;

use crate::aoc_lib::*;

pub fn solve_18a(input: &str) -> usize {
    let digs = input.lines().map(Dig::from_a).collect::<Vec<_>>();

    let mut dig_pos = p!(0, 0);
    let mut dig_positions = Vec::new();

    for dig in digs {
        for _ in 0..dig.length {
            dig_pos = dig_pos.position_in_direction(dig.direction, 1);
            dig_positions.push(dig_pos);
        }
    }

    let min_x = dig_positions.iter().map(|pos| pos.x).min().unwrap();
    let max_x = dig_positions.iter().map(|pos| pos.x).max().unwrap();
    let min_y = dig_positions.iter().map(|pos| pos.y).min().unwrap();
    let max_y = dig_positions.iter().map(|pos| pos.y).max().unwrap();

    let polygon = Polygon::new(LineString::new(
        dig_positions
            .iter()
            .map(|pos| {
                let mut c = Coord::zero();
                c.x = pos.x as f32;
                c.y = pos.y as f32;
                c
            })
            .collect()
    ), vec![]);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = p!(x, y);
            if !dig_positions.contains(&pos) && polygon.contains(&point!(x: pos.x as f32, y: pos.y as f32)) {
                dig_positions.push(pos);
            }
        }
    }

    dig_positions.len()
}

pub fn solve_18b(input: &str) -> u128 {
    let digs = input.lines().map(Dig::from_b).collect::<Vec<_>>();

    let mut dig_pos = p!(0, 0);
    let mut positions = Vec::with_capacity(digs.len() + 1);

    let length_sum: usize = digs.iter().map(|d| d.length).sum::<usize>();

    println!("{length_sum}");

    for dig in digs {
        let length = dig.length as isize;

        match dig.direction {
            XP => dig_pos.x += length,
            XM => dig_pos.x -= length,
            YP => dig_pos.y += length,
            YM => dig_pos.y -= length,
            _ => {}
        }

        positions.push(dig_pos)
    }

    let mut sum = 0_i128;

    for i in 0..positions.len() {
        let next_i = if i == positions.len() - 1 {
            0
        } else {
            i + 1
        };
        let pos = positions[i];
        let next_pos = positions[next_i];

        sum += ((pos.y + next_pos.y) * (pos.x - next_pos.x)) as i128
    }

    positions.iter().for_each(|pos| println!("{:?}", pos));

    (sum / 2).abs() as u128
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: usize,
}

impl Dig {
    fn from_a(line: &str) -> Self {
        let split = line.split(" ").collect::<Vec<_>>();

        let direction = match split[0] {
            "U" => YM,
            "D" => YP,
            "L" => XM,
            "R" => XP,
            _ => panic!("invalid")
        };

        let length = split[1].parse::<usize>().unwrap();

        Dig {
            direction,
            length,
        }
    }

    fn from_b(line: &str) -> Self {
        let color = line.split(" ").last().unwrap().replace("(", "").replace(")", "").replace("#", "");

        let length = usize::from_str_radix(&color[0..5], 16).unwrap();
        let direction = match &color[5..=5] {
            "0" => XP,
            "1" => YP,
            "2" => XM,
            "3" => YM,
            _ => panic!("invalid")
        };

        Dig {
            length, direction
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Trench(Color),
    Ground,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Trench(Black),
            '.' => Ground,
            _ => panic!("invalid")
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Trench(_) => '#',
            Ground => '.'
        }
    }
}