use std::collections::HashMap;
use crate::y2024::d19::Color::*;

pub fn solve_a(input: &str) -> usize {
    let mut split = input.split("\n\n");

    let towels = split
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(Towel::new)
        .collect::<Vec<_>>();

    split
        .next()
        .unwrap()
        .lines()
        .map(Pattern::new)
        .filter(|p| p.possible_with(&towels))
        .count()
}

pub fn solve_b(input: &str) -> usize {
    let mut split = input.split("\n\n");

    let towels = split
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(Towel::new)
        .collect::<Vec<_>>();

    split
        .next()
        .unwrap()
        .lines()
        .map(Pattern::new)
        .filter(|p| p.possible_with(&towels))
        .map(|p| p.num_possible_arrangements(&towels))
        .sum()
}

#[derive(Debug)]
struct Pattern {
    colors: Vec<Color>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        Pattern {
            colors: input.chars().map(Color::from).collect()
        }
    }

    fn possible_with(&self, towels: &Vec<Towel>) -> bool {
        let mut selected = vec![];

        self.possible_with_rec(0, towels, &mut selected)
    }

    fn possible_with_rec(
        &self,
        index: usize,
        towels: &Vec<Towel>,
        selection: &mut Vec<Towel>
    ) -> bool {
        if index == self.len() {
            // it was possible to add towels until the length of the pattern was reached, so
            // it is possible to create this pattern with the given towels. Return true
            return true;
        }

        for towel in towels {
            if self.index_matches_towel(index, towel) {
                // the towel can be added, add it to the selection
                selection.push(towel.clone());

                if self.possible_with_rec(index + towel.len(), towels, selection) {
                    // going deeper, it was possible to create the pattern with the towels,
                    // so return true
                    return true;
                } else {
                    // adding the towel to the selection did not produce the pattern in the long run,
                    // so remove the towel from the selection and try the next towel (backtrack)
                    selection.pop();
                }
            }
        }

        // no towel could be added to the selection, as none matches the pattern
        // at the current index
        false
    }

    fn num_possible_arrangements(&self, towels: &Vec<Towel>) -> usize {
        let mut selection = vec![];
        let mut cache = HashMap::new();
        self.num_possible_arrangements_rec(0, towels, &mut selection, &mut cache)
    }

    fn num_possible_arrangements_rec<'a>(
        &'a self,
        index: usize,
        towels: &Vec<Towel>,
        selection: &mut Vec<Towel>,
        cache: &mut HashMap<&'a [Color], usize>,
    ) -> usize {
        if index == self.len() {
            // it was possible to add towels until the length of the pattern was reached, so
            // it is possible to create this pattern with the current selection. Add 1
            return 1;
        }

        // the key of the cache are the remaining colors from index to the end of this pattern
        let remaining = &self.colors[index..];

        match cache.get(remaining) {
            // the cache contains the remaining amount, just return this
            Some(amount) => *amount,
            None => {
                // the cache does not contain the remaining amounts,
                // so they must be calculated for all the towels
                let mut sum = 0;

                for towel in towels {
                    if self.index_matches_towel(index, towel) {
                        // temporarily add the towel to the selection
                        selection.push(towel.clone());

                        // calculate  the amount for the towel and store its value in the cache
                        // (&self.colors[index + towel.len()..], as these are the remaining colors after the towel was added)
                        let num = self.num_possible_arrangements_rec(index + towel.len(), towels, selection, cache);
                        cache.insert(&self.colors[index + towel.len()..], num);
                        sum += num;

                        // remove the towel from the selection, preparing it for
                        // the next one
                        selection.pop();
                    }
                }

                sum
            }
        }
    }

    fn index_matches_towel(&self, index: usize, towel: &Towel) -> bool {
        if index + towel.len() <= self.len() {
            (0..towel.len()).into_iter().all(|i| self.colors[index + i] == towel.colors[i])
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.colors.len()
    }
}

#[derive(Clone, Debug)]
struct Towel {
    colors: Vec<Color>,
}

impl Towel {
    fn new(input: &str) -> Self {
        Towel {
            colors: input.chars().map(Color::from).collect()
        }
    }

    fn len(&self) -> usize {
        self.colors.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => White,
            'u' => Blue,
            'b' => Black,
            'r' => Red,
            'g' => Green,
            _ => unreachable!()
        }
    }
}