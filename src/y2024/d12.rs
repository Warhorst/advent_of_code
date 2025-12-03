use crate::aoc_lib::*;
use std::collections::HashSet;
use std::fmt::Formatter;

pub fn solve_a(input: &str) -> usize {
    collect_plots(input)
        .into_iter()
        .map(|plot| plot.size() * plot.fences())
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    collect_plots(input)
        .into_iter()
        .map(|plot| plot.size() * plot.sides())
        .sum()
}

fn collect_plots(input: &str) -> Vec<Plot> {
    let mut plots: Vec<Plot> = vec![];

    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, plant)| {
            let pos = p!(x, y);
            match plots.iter_mut().find(|plot| plot.plant_belongs_to_plot(plant, pos)) {
                Some(plot) => plot.add_plant(pos),
                None => plots.push(Plot::new(plant, pos))
            }
        }));

    loop {
        let mut merged: Vec<Plot> = vec![];

        for plot in plots.iter() {
            match merged.iter_mut().find(|p| p.plot_belongs_to_plot(plot)) {
                Some(p) => p.add_plot(plot.clone()),
                None => merged.push(plot.clone())
            }
        }

        if merged.len() < plots.len() {
            plots = merged;
        } else {
            break;
        }
    }

    plots
}

#[derive(Clone)]
struct Plot {
    plant: char,
    positions: HashSet<Position>,
}

impl Plot {
    fn new(plant: char, init_pos: Position) -> Self {
        Plot {
            plant,
            positions: HashSet::from_iter([init_pos]),
        }
    }

    fn plant_belongs_to_plot(&self, plant: char, pos: Position) -> bool {
        self.plant == plant && self.positions.iter().any(|p| p.is_cardinal_neighbour_with(&pos))
    }

    fn add_plant(&mut self, pos: Position) {
        self.positions.insert(pos);
    }

    fn plot_belongs_to_plot(&self, other: &Plot) -> bool {
        self.plant == other.plant && self.positions.iter().any(|self_pos| other.positions.iter().any(|other_pos| self_pos.is_cardinal_neighbour_with(other_pos)))
    }

    fn add_plot(&mut self, plot: Plot) {
        self.positions.extend(plot.positions)
    }

    fn size(&self) -> usize {
        self.positions.len()
    }

    fn fences(&self) -> usize {
        // the fences are just all cardinal neighbours which are not part of the actual plot
        // duplicates are allowed, as every plant needs a fence in every direction
        self.positions
            .iter()
            .map(|pos| pos
                .cardinal_neighbours()
                .into_iter()
                .filter(|n| !self.positions.contains(n))
                .count()
            )
            .sum()
    }

    fn sides(&self) -> usize {
        // the sides are equal to the plots: I start by transforming
        // everything outside the plot into a one tile side and then merge
        // the sides until no more changes occur
        let mut sides = self.positions
            .iter()
            .flat_map(|pos| pos
                .cardinal_neighbours_with_directions()
                .into_iter()
                .filter(|(n, _)| !self.positions.contains(n))
                .map(|(n, d)| Side::new(d, n))
            )
            .collect::<Vec<_>>();

        loop {
            let mut merged: Vec<Side> = vec![];

            for side in sides.iter() {
                match merged.iter_mut().find(|s| s.side_belongs_to_side(side)) {
                    Some(s) => s.add_side(side.clone()),
                    None => merged.push(side.clone())
                }
            }

            if merged.len() < sides.len() {
                sides = merged;
            } else {
                break;
            }
        }

        sides.len()
    }
}

impl std::fmt::Display for Plot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, size {}, fences: {}, sides: {}", self.plant, self.size(), self.fences(), self.sides())
    }
}

#[derive(Clone)]
struct Side {
    dir: Direction,
    positions: HashSet<Position>
}

impl Side {
    fn new(dir: Direction, pos: Position) -> Self {
        let mut positions = HashSet::new();
        positions.insert(pos);

        Side {
            dir,
            positions
        }
    }

    fn side_belongs_to_side(&self, other: &Side) -> bool {
        self.dir == other.dir && self.positions.iter().any(|self_pos| other.positions.iter().any(|other_pos| self_pos.is_cardinal_neighbour_with(other_pos)))
    }

    fn add_side(&mut self, plot: Side) {
        self.positions.extend(plot.positions)
    }
}
