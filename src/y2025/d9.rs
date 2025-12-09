use std::cmp::Ordering;

use geo::{Contains, LineString, Polygon, coord};
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let positions = input
        .lines()
        .map(|line| {
            let split = line.split_once(",").unwrap();
            (parse::<usize>(split.0), parse::<usize>(split.1))
        })
        .collect::<Vec<_>>();

    let mut combinations = vec![];

    for i in 0..positions.len() {
        for j in i..positions.len() {
            combinations.push((positions[i], positions[j]));
        }
    }

    combinations
        .into_iter()
        .map(|((xa, ya), (xb, yb))| (xb.abs_diff(xa) + 1) * (yb.abs_diff(ya) + 1))
        .max()
        .unwrap()
}

pub fn solve_b(input: &str) -> usize {
    let positions = input
        .lines()
        .map(|line| {
            let split = line.split_once(",").unwrap();
            (parse::<usize>(split.0), parse::<usize>(split.1))
        })
        .map(|(x, y)| (x as f32, y as f32))
        .collect::<Vec<_>>();

    let mut combinations = vec![];
    for i in 0..positions.len() {
        for j in i..positions.len() {
            combinations.push((positions[i], positions[j]));
        }
    }

    let s = LineString::new(
        positions
            .iter()
            .map(|(x, y)| coord!(x: *x, y: *y))
            .collect(),
    );
    let polygon = Polygon::new(s, vec![]);

    combinations
        .into_iter()
        .par_bridge() // rayon for speed
        .map(|((xa, ya), (xb, yb))| {
            (
                ((xa, ya), (xb, yb)),
                // Create polygon sqares based on the given combinations
                Polygon::new(
                    match xa.partial_cmp(&xb).unwrap() {
                        Ordering::Less => match ya.partial_cmp(&yb).unwrap() {
                            Ordering::Less => LineString(vec![
                                coord! {x: xa, y: ya},
                                coord! {x: xb, y: ya},
                                coord! {x: xb, y: yb},
                                coord! {x: xa, y: yb},
                            ]),
                            Ordering::Equal => LineString::new(vec![
                                coord! {x: xa, y: xb},
                                coord! {x: xb, y: yb},
                            ]),
                            Ordering::Greater => LineString(vec![
                                coord! {x: xa, y: ya},
                                coord! {x: xa, y: yb},
                                coord! {x: xb, y: yb},
                                coord! {x: xb, y: ya},
                            ]),
                        },
                        Ordering::Equal => LineString::new(vec![
                            coord! {x: xa, y: xb},
                            coord! {x: xb, y: yb},
                        ]),
                        Ordering::Greater => match ya.partial_cmp(&yb).unwrap() {
                            Ordering::Less => LineString(vec![
                                coord! {x: xa, y: ya},
                                coord! {x: xa, y: yb},
                                coord! {x: xb, y: yb},
                                coord! {x: xb, y: ya},
                            ]),
                            Ordering::Equal => LineString::new(vec![
                                coord! {x: xa, y: xb},
                                coord! {x: xb, y: yb},
                            ]),
                            Ordering::Greater => LineString(vec![
                                coord! {x: xa, y: ya},
                                coord! {x: xb, y: ya},
                                coord! {x: xb, y: yb},
                                coord! {x: xa, y: yb},
                            ]),
                        },
                    },
                    vec![],
                ),
            )
        })
        .filter(|(_, sqare)| polygon.contains(sqare))
        .map(|(((xa, ya), (xb, yb)), _)| ((xa as usize, ya as usize), (xb as usize, yb as usize)))
        .map(|((xa, ya), (xb, yb))| (xb.abs_diff(xa) + 1) * (yb.abs_diff(ya) + 1))
        .max()
        .unwrap()
}
