use std::collections::HashMap;
use std::fmt::Formatter;
use std::thread::sleep;
use std::time::Duration;
use regex::Regex;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let dim_regex = Regex::new(r#"(\d+),(\d+)"#).unwrap();
    let robot_regex = Regex::new(r#"p=(\d+),(\d+)\sv=(-?\d+),(-?\d+)"#).unwrap();

    let dim_line = input.lines().next().unwrap();
    let dim: (isize, isize) = regex_captures(
        dim_line,
        &dim_regex,
        |caps| (parse(caps[0]), parse(caps[1])),
    ).into_iter().next().unwrap();

    let robot_input = input
        .lines()
        .skip(1)
        .collect::<String>();

    let q1 = Bounds::new(0, 0, dim.0 / 2 - 1, dim.1 / 2 - 1);
    let q2 = Bounds::new(0, dim.1 / 2 + 1, dim.0 / 2 - 1, dim.1 - 1);
    let q3 = Bounds::new(dim.0 / 2 + 1, 0, dim.0 - 1, dim.1 / 2 - 1);
    let q4 = Bounds::new(dim.0 / 2 + 1, dim.1 / 2 + 1, dim.0 - 1, dim.1 - 1);

    let res = regex_captures(
        &robot_input,
        &robot_regex,
        |caps| Robot {
            start: p!(parse::<isize>(caps[0]), parse::<isize>(caps[1])),
            vel: (parse::<isize>(caps[2]), parse::<isize>(caps[3])),
        },
    )
        .into_iter()
        .map(|r| (0..100)
            .into_iter()
            .fold(r.start, |acc, _| {
                let mut new_pos = acc + r.vel;

                if new_pos.x < 0 {
                    new_pos.x += dim.0
                }

                if new_pos.x >= dim.0 {
                    new_pos.x -= dim.0
                }

                if new_pos.y < 0 {
                    new_pos.y += dim.1
                }

                if new_pos.y >= dim.1 {
                    new_pos.y -= dim.1
                }

                new_pos
            })
        )
        .fold((0, 0, 0, 0), |acc, item| {
            if q1.contains_position(item) {
                (acc.0 + 1, acc.1, acc.2, acc.3)
            } else if q2.contains_position(item) {
                (acc.0, acc.1 + 1, acc.2, acc.3)
            } else if q3.contains_position(item) {
                (acc.0, acc.1, acc.2 + 1, acc.3)
            } else if q4.contains_position(item) {
                (acc.0, acc.1, acc.2, acc.3 + 1)
            } else {
                acc
            }
        });

    res.0 * res.1 * res.2 * res.3
}

pub fn solve_b(input: &str) -> usize {
    // solving it the most stupid way: I looked on reddit how this tree should look
    // and printed every position that might represent it
    let dim_regex = Regex::new(r#"(\d+),(\d+)"#).unwrap();
    let robot_regex = Regex::new(r#"p=(\d+),(\d+)\sv=(-?\d+),(-?\d+)"#).unwrap();

    let dim_line = input.lines().next().unwrap();
    let dim: (isize, isize) = regex_captures(
        dim_line,
        &dim_regex,
        |caps| (parse(caps[0]), parse(caps[1])),
    ).into_iter().next().unwrap();

    let robot_input = input
        .lines()
        .skip(1)
        .collect::<String>();

    let robots = regex_captures(
        &robot_input,
        &robot_regex,
        |caps| Robot {
            start: p!(parse::<isize>(caps[0]), parse::<isize>(caps[1])),
            vel: (parse::<isize>(caps[2]), parse::<isize>(caps[3])),
        },
    ).into_iter().collect::<Vec<_>>();

    let mut positions = robots.iter().map(|r| r.start).collect::<Vec<_>>();
    let mut count = 0;

    loop {
        count += 1;

        positions = positions.into_iter().zip(robots.iter()).map(|(pos, r)| {
            let mut new_pos = pos + r.vel;

            if new_pos.x < 0 {
                new_pos.x += dim.0
            }

            if new_pos.x >= dim.0 {
                new_pos.x -= dim.0
            }

            if new_pos.y < 0 {
                new_pos.y += dim.1
            }

            if new_pos.y >= dim.1 {
                new_pos.y -= dim.1
            }

            new_pos
        }).collect();

        let mut y_to_x_amount_map = HashMap::new();

        positions.iter().for_each(|pos| { y_to_x_amount_map.entry(pos.y).and_modify(|amount| *amount += 1).or_insert(0);});

        if y_to_x_amount_map.values().filter(|amount| **amount == 32).count() == 2 {
            println!("{count}");
            PositionPrinter::new().draw_axis(false).print(positions.iter().copied());
            sleep(Duration::from_secs_f64(1.0))
        }
    }

    0
}

struct Robot {
    start: Position,
    vel: (isize, isize),
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Start: {:?}, Velocity: {:?}", self.start, self.vel)
    }
}