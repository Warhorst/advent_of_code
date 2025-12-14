use std::collections::HashMap;

use bitarray::BitArray;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    input.lines().map(Machine::from).map(|m| m.activate()).sum()
}

pub fn solve_b(input: &str) -> usize {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.set_joltage())
        .sum::<usize>()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
struct Array {
    bits: BitArray<u16>,
    len: u8,
}

impl Array {
    fn new(
        num: u16,
        len: u8,
    ) -> Self {
        Array {
            bits: BitArray::new(num),
            len,
        }
    }

    fn push(
        &mut self,
        val: bool,
    ) {
        self.bits.set(self.len, val);
        self.len += 1
    }

    fn set_len(
        &mut self,
        len: u8,
    ) {
        self.len = len;
    }

    fn set_true(
        &mut self,
        index: u8,
    ) {
        self.bits.set(index, true);
    }

    fn apply(
        &self,
        other: Array,
    ) -> Array {
        let mut new = *self;

        for (i, val) in self.bits.iter().enumerate().take(self.len as usize) {
            if other.bits.get(i as u8) {
                new.bits.set(i as u8, !val);
            }
        }

        new
    }
}

#[derive(Debug)]
struct Machine {
    lights: Array,
    buttons: Vec<Array>,
    joltage: Vec<usize>,
}

impl Machine {
    fn activate(&self) -> usize {
        // All the possible states of the lights
        let possibilities = (0..2usize.pow(self.lights.len as u32))
            .map(|num| Array::new(num as u16, self.lights.len))
            .collect::<Vec<_>>();

        let mut edges = HashMap::<(Array, Array), Array>::new();

        for p in &possibilities {
            for b in &self.buttons {
                edges.insert((*p, *b), p.apply(*b));
            }
        }

        let start = Array::new(0, self.lights.len);

        let res = dijkstra(
            &start,
            |a| {
                self.buttons
                    .iter()
                    .flat_map(|p| edges.get(&(*a, *p)))
                    .map(|edge| (*edge, 1))
                    .collect::<Vec<_>>()
            },
            |a| *a == self.lights,
        );

        res.unwrap().1
    }

    fn set_joltage(&self) -> usize {
        // Solve as a linear programming problem.
        // As I had no idea how to do this in rust, I followed this solution which uses microlp: https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nta10wc/

        let buttons = self
            .buttons
            .iter()
            .map(|b| {
                b.bits
                    .iter()
                    .take(self.lights.len as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // The linear programming problem to solve. In this case, we want the minimum result.
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        // Create the variables for each joltage. Every joltage level has one variable to determine.
        let vars = (0..buttons.len())
            .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
            .collect::<Vec<_>>();

        for j in 0..self.joltage.len() {
            // The linear expression left hand side for the current joltage
            let mut expression = LinearExpr::empty();

            for b in 0..buttons.len() {
                if buttons[b][j] {
                    // For every button, add a variable if it effects the current joltage.
                    // We can call buttons[b] 'x_0'. We can interpret the buttons as a matrix, where
                    // the columns are the joltage levels the button effects and the rows are the equation
                    // which should result in the joltage level. When updating the expression
                    // for all button levels and if 3 buttons effect the joltage, than the left
                    // hand side of the linear expression will be 1.0 * x_0 + 1.0 * x_0 + 1.0 * x_0.
                    expression.add(vars[b], 1.0);
                }
            }

            // Continuing the comment from avove: This call now defines what the expression should resolve in.
            // In this case, the whole expression is 1.0 * x_0 + 1.0 * x_0 + 1.0 * x_0 = joltage[j] .
            problem.add_constraint(expression, ComparisonOp::Eq, self.joltage[j] as f64);
        }

        // The optimizer searches for the vector which value-sum is the lowest, which is already
        // what we want. So we just return the objective result, converted to a usize.
        problem.solve().unwrap().objective().round() as usize
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lights = Array::default();
        let mut buttons = vec![];
        let mut joltage = vec![];

        for s in value.split(" ") {
            if s.starts_with("[") {
                s.chars()
                    .filter(|c| *c != '[' && *c != ']')
                    .map(|c| c != '.')
                    .for_each(|b| lights.push(b));
            } else if s.starts_with("(") {
                let val = s.replace("(", "").replace(")", "");
                let button_values = val.split(",").map(parse::<u8>).collect::<Vec<_>>();
                let mut button = Array::default();
                button.set_len(button_values.len() as u8);
                button_values.into_iter().for_each(|v| button.set_true(v));
                buttons.push(button);
            } else {
                let val = s.replace("{", "").replace("}", "");
                val.split(",")
                    .map(parse::<usize>)
                    .for_each(|v| joltage.push(v));
            }
        }

        Machine {
            lights,
            buttons,
            joltage,
        }
    }
}
