use std::{cmp::Ordering, collections::HashSet};

use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    // Kruskals algorithm, but limit it to a specific number of iterations (10 in the example, 1000 in the puzzle)

    let positions = input.lines().map(Pos::new).collect::<Vec<_>>();

    let mut edges = Vec::with_capacity((positions.len() * (positions.len() - 1)) / 2);
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            edges.push((positions[i], positions[j]));
        }
    }
    edges.sort_by(|a, b| a.0.distance(&a.1).partial_cmp(&b.0.distance(&b.1)).unwrap());

    let mut circuits: Vec<HashSet<Pos>> = vec![];
    for pos in &positions {
        let mut c = HashSet::new();
        c.insert(*pos);
        circuits.push(c);
    }

    let iterations = if positions.len() == 1000 {
        // Puzzle
        1000
    } else {
        // example A
        10
    };

    for edge in edges.into_iter().take(iterations) {
        let ia = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&edge.0) { Some(i) } else { None })
            .unwrap();
        let ib = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&edge.1) { Some(i) } else { None })
            .unwrap();

        match ia.cmp(&ib) {
            Ordering::Less => {
                let b = circuits.remove(ib);
                let a = circuits.get_mut(ia).unwrap();
                b.into_iter().for_each(|p| {
                    a.insert(p);
                });
            }
            Ordering::Equal => continue,
            Ordering::Greater => {
                let a = circuits.remove(ia);
                let b = circuits.get_mut(ib).unwrap();
                a.into_iter().for_each(|p| {
                    b.insert(p);
                });
            }
        }
    }

    let mut lens = circuits.into_iter().map(|c| c.len()).collect::<Vec<_>>();
    lens.sort_by(|a, b| b.cmp(a));
    lens[0] * lens[1] * lens[2]
}

pub fn solve_b(input: &str) -> usize {
    // Just Kruskals algorithm

    let positions = input.lines().map(Pos::new).collect::<Vec<_>>();

    let mut edges = Vec::with_capacity((positions.len() * (positions.len() - 1)) / 2);
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            edges.push((positions[i], positions[j]));
        }
    }
    edges.sort_by(|a, b| a.0.distance(&a.1).partial_cmp(&b.0.distance(&b.1)).unwrap());

    let mut circuits: Vec<HashSet<Pos>> = vec![];
    for pos in &positions {
        let mut c = HashSet::new();
        c.insert(*pos);
        circuits.push(c);
    }

    for edge in edges {
        let ia = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&edge.0) { Some(i) } else { None })
            .unwrap();
        let ib = circuits
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.contains(&edge.1) { Some(i) } else { None })
            .unwrap();

        match ia.cmp(&ib) {
            Ordering::Less => {
                let b = circuits.remove(ib);
                let a = circuits.get_mut(ia).unwrap();
                b.into_iter().for_each(|p| {
                    a.insert(p);
                });
            }
            Ordering::Equal => continue,
            Ordering::Greater => {
                let a = circuits.remove(ia);
                let b = circuits.get_mut(ib).unwrap();
                a.into_iter().for_each(|p| {
                    b.insert(p);
                });
            }
        }

        if circuits.len() == 1 {
            return edge.0.x * edge.1.x;
        }
    }

    panic!("A solution should have been found.")
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
