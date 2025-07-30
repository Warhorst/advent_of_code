use std::collections::HashSet;
use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let input = parse_input(input);

    let mut tail_pos = p!(0, 0);
    let mut head_pos = p!(0, 0);
    let mut visited = HashSet::new();

    for dir in input {
        head_pos = head_pos.position_in_direction(dir, 1);

        let diff = head_pos - tail_pos;

        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            tail_pos = match (diff.x, diff.y) {
                (x, y) if x > 0 && y == 0 => tail_pos.position_in_direction(XP, 1),
                (x, y) if x < 0 && y == 0 => tail_pos.position_in_direction(XM, 1),
                (x, y) if x == 0 && y > 0 => tail_pos.position_in_direction(YP, 1),
                (x, y) if x == 0 && y < 0 => tail_pos.position_in_direction(YM, 1),
                (x, y) if x > 0 && y > 0 => tail_pos.position_in_direction(XPYP, 1),
                (x, y) if x > 0 && y < 0 => tail_pos.position_in_direction(XPYM, 1),
                (x, y) if x < 0 && y > 0 => tail_pos.position_in_direction(XMYP, 1),
                (x, y) if x < 0 && y < 0 => tail_pos.position_in_direction(XMYM, 1),
                _ => tail_pos
            };
        }

        visited.insert(tail_pos);
    }

    visited.len()
}

pub fn solve_b(input: &str) -> usize {
    let input = parse_input(input);
    let mut knots = [p!(0, 0); 10];

    let mut visited = HashSet::new();

    for dir in input {
        knots[0] = knots[0].position_in_direction(dir, 1);

        for i in 1..10 {
            let head_pos = knots[i - 1];
            let tail_pos = knots.get_mut(i).unwrap();
            let diff = head_pos - *tail_pos;

            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                *tail_pos = match (diff.x, diff.y) {
                    (x, y) if x > 0 && y == 0 => tail_pos.position_in_direction(XP, 1),
                    (x, y) if x < 0 && y == 0 => tail_pos.position_in_direction(XM, 1),
                    (x, y) if x == 0 && y > 0 => tail_pos.position_in_direction(YP, 1),
                    (x, y) if x == 0 && y < 0 => tail_pos.position_in_direction(YM, 1),
                    (x, y) if x > 0 && y > 0 => tail_pos.position_in_direction(XPYP, 1),
                    (x, y) if x > 0 && y < 0 => tail_pos.position_in_direction(XPYM, 1),
                    (x, y) if x < 0 && y > 0 => tail_pos.position_in_direction(XMYP, 1),
                    (x, y) if x < 0 && y < 0 => tail_pos.position_in_direction(XMYM, 1),
                    _ => *tail_pos
                };
            }

            visited.insert(knots[9]);
        }

    }

    visited.len()
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            (split.next().unwrap().chars().next().unwrap(), parse::<usize>(split.next().unwrap()))
        })
        .flat_map(|(dir, amount)| {
            let dir = match dir {
                'R' => XP,
                'L' => XM,
                'U' => YP,
                'D' => YM,
                _ => panic!("invalid char")
            };

            (0..amount).into_iter().map(move |_| dir)
        })
        .collect()
}