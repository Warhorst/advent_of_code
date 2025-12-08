use std::collections::{HashMap, HashSet};

use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let positions = input.lines().map(Pos::new).collect::<Vec<_>>();

    let mut all_distances = positions
        .iter()
        .flat_map(|pos| positions.iter().map(|p| (*pos, *p)))
        .filter(|pair| pair.0 != pair.1)
        .map(|pair| (pair, pair.0.distance(&pair.1)))
        .collect::<HashMap<_, _>>();

    let mut circuits: Vec<HashSet<Pos>> = vec![];

    for pos in &positions {
        let mut c = HashSet::new();
        c.insert(*pos);
        circuits.push(c);
    }

    // todo add the number of iterations as part of the puzzle input
    // todo pretty slow
    for _ in 0..positions.len() {
        let pair = all_distances
            .iter()
            .map(|val| (*val.0, *val.1))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        all_distances.remove(&pair);
        all_distances.remove(&(pair.1, pair.0));

        let already_contained = circuits
            .iter()
            .any(|c| c.contains(&pair.0) && c.contains(&pair.1));

        if already_contained {
            continue;
        }

        let ia = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&pair.0) { Some(i) } else { None })
            .unwrap();
        let ib = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&pair.1) { Some(i) } else { None })
            .unwrap();

        if ia > ib {
            let a = circuits.remove(ia);
            let b = circuits.remove(ib);
            circuits.push(&a | &b);
        } else {
            let b = circuits.remove(ib);
            let a = circuits.remove(ia);
            circuits.push(&a | &b);
        }
    }

    // circuits.iter().for_each(|c| println!("{}: {c:?}", c.len()));

    let mut lens = circuits.into_iter().map(|c| c.len()).collect::<Vec<_>>();
    lens.sort_by(|a, b| b.cmp(a));
    lens[0] * lens[1] * lens[2]
    // 42
}

pub fn solve_b(input: &str) -> usize {
    let positions = input.lines().map(Pos::new).collect::<Vec<_>>();

    let mut all_distances = positions
        .iter()
        .flat_map(|pos| positions.iter().map(|p| (*pos, *p)))
        .filter(|pair| pair.0 != pair.1)
        .map(|pair| (pair, pair.0.distance(&pair.1)))
        .collect::<HashMap<_, _>>();

    let mut circuits: Vec<HashSet<Pos>> = vec![];

    for pos in &positions {
        let mut c = HashSet::new();
        c.insert(*pos);
        circuits.push(c);
    }

    // todo holy shit this is even slower
    let pair = loop {
        let pair = all_distances
            .iter()
            .map(|val| (*val.0, *val.1))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        all_distances.remove(&pair);
        all_distances.remove(&(pair.1, pair.0));

        let already_contained = circuits
            .iter()
            .any(|c| c.contains(&pair.0) && c.contains(&pair.1));

        if already_contained {
            continue;
        }

        let ia = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&pair.0) { Some(i) } else { None })
            .unwrap();
        let ib = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&pair.1) { Some(i) } else { None })
            .unwrap();

        if ia > ib {
            let a = circuits.remove(ia);
            let b = circuits.remove(ib);
            circuits.push(&a | &b);
        } else {
            let b = circuits.remove(ib);
            let a = circuits.remove(ia);
            circuits.push(&a | &b);
        }

        if circuits.len() == 1 {
            break pair;
        }
    };

    pair.0.x * pair.1.x
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    fn new(line: &str) -> Self {
        let mut split = line.split(",");

        Pos {
            x: parse(split.next().unwrap()),
            y: parse(split.next().unwrap()),
            z: parse(split.next().unwrap()),
        }
    }

    fn distance(
        &self,
        other: &Pos,
    ) -> f32 {
        (((self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)) as f32)
            .sqrt()
    }
}
