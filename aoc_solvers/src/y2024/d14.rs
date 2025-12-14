use helpers::prelude::*;
use indoc::indoc;
use regex::Regex;
use std::fmt::Formatter;

pub fn solve_a(input: &str) -> usize {
    // I manually added the dimension of the board to my puzzle/example input to vary between the 2
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
    let tree = tree_shape();
    tree.print();

    // For every iteration of robots, create a board from their current positions
    // and check if the shape is in there
    // a bit slow, but it works
    loop {
        count += 1;
        robots
            .iter()
            .enumerate()
            .for_each(|(i, r)| {
                let mut new_pos = positions[i] + r.vel;

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

                *positions.get_mut(i).unwrap() = new_pos
            });

        let board = Board::from_positions_and_bounds(
            positions.iter().copied(),
            dim.0 as usize,
            dim.1 as usize,
            || true,
            || false
        );

        if board.contains_shape(&tree, true) {
            break count
        }
    }
}

fn tree_shape() -> Shape {
    let tree_string = indoc! {"
        XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
        X                             X
        X                             X
        X                             X
        X                             X
        X              X              X
        X             XXX             X
        X            XXXXX            X
        X           XXXXXXX           X
        X          XXXXXXXXX          X
        X            XXXXX            X
        X           XXXXXXX           X
        X          XXXXXXXXX          X
        X         XXXXXXXXXXX         X
        X        XXXXXXXXXXXXX        X
        X          XXXXXXXXX          X
        X         XXXXXXXXXXX         X
        X        XXXXXXXXXXXXX        X
        X       XXXXXXXXXXXXXXX       X
        X      XXXXXXXXXXXXXXXXX      X
        X        XXXXXXXXXXXXX        X
        X       XXXXXXXXXXXXXXX       X
        X      XXXXXXXXXXXXXXXXX      X
        X     XXXXXXXXXXXXXXXXXXX     X
        X    XXXXXXXXXXXXXXXXXXXXX    X
        X             XXX             X
        X             XXX             X
        X             XXX             X
        X                             X
        X                             X
        X                             X
        X                             X
        XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    "};

    Shape::from(tree_string)
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
