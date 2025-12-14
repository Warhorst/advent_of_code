use std::str::FromStr;
use rayon::iter::ParallelIterator;
use rayon::iter::ParallelBridge;
use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    // solve with simple brute force
    input
        .lines()
        .map(|line| Input::from_str(line).unwrap())
        .filter(fulfills_equation_2_ops)
        .map(|input| input.result)
        .sum()
}

fn fulfills_equation_2_ops(input: &Input) -> bool {
    // create an iterator of all possible permutations
    // as there are only 2 operators, a simple number (which is basically a bool array) is sufficient
    permutations_2_ops(input.nums.len())
        .into_iter()
        .any(|perm| {
            input.nums
                .iter()
                .enumerate()
                .fold(0, |acc, (i, num)| {
                    // if i is 0, the fold just started, and the first
                    // element is set as current accumulator
                    if i == 0 {
                        *num
                    } else {
                        // i - 1, as the first index was skipped
                        // (perm & (1 << (i - 1))) = check if the bit at i - 1 is set
                        // >> (i - 1) = shift the bit at i - 1 to the right with i - 1, returning either 0 (not set) or 1 (set)
                        match (perm & (1 << (i - 1))) >> (i - 1) {
                            0 => acc + *num,
                            1 => acc * *num,
                            _ => unreachable!()
                        }
                    }
                }) == input.result
        })
}

fn permutations_2_ops(num_len: usize) -> impl IntoIterator<Item=usize> {
    0..2_usize.pow((num_len - 1) as u32)
}

pub fn solve_b(input: &str) -> usize {
    // solve with simple brute force, again
    input
        .lines()
        .par_bridge() // rayon for speed
        .map(|line| Input::from_str(line).unwrap())
        .filter(fulfills_equation_3_ops)
        .map(|input| input.result)
        .sum()
}

fn fulfills_equation_3_ops(input: &Input) -> bool {
    // Basically the same as before, but with 3 operators for now.
    // This code works under the assumption that there are at max 11 operators in an equation,
    // which was the case for my input. As before, I use numbers to quickly create permutations,
    // but 2 bits are now required to represent an operator
    permutations_3_ops(input.nums.len())
        .into_iter()
        .any(|perm| {
            input.nums
                .iter()
                .enumerate()
                .fold(0, |acc, (i, num)| {
                    if i == 0 {
                        *num
                    } else {
                        // bit-value at the index representing either add or mul
                        let add_or_mul = (perm & (1 << (i * 2 - 1))) >> (i * 2 - 1);
                        // bit-value at the index representing concat
                        let cc = (perm & (1 << (i * 2))) >> (i * 2);

                        match (add_or_mul, cc) {
                            // if concat is 0, perform add or mul
                            (am, 0) => match am {
                                0 => acc + *num,
                                1 => acc * *num,
                                _ => unreachable!()
                            },
                            // if concat is 1, well... concat
                            (_, 1) => concat(acc, *num),
                            _ => unreachable!()
                        }
                    }
                }) == input.result
        })
}

fn permutations_3_ops(num_len: usize) -> impl IntoIterator<Item=usize> {
    0..2_usize.pow((num_len * 2 - 1) as u32)
}

fn concat(a: usize, b: usize) -> usize {
    let places = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_u32.pow(places) as usize + b
}

struct Input {
    result: usize,
    nums: Vec<usize>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let result = parse(split.next().unwrap().trim());

        let left = split.next().unwrap();
        let nums = left
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(parse)
            .collect();

        Ok(Input {
            result,
            nums,
        })
    }
}
