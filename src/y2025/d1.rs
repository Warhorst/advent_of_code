use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    let mut dial = 50;
    let mut times_0 = 0;

    input
        .lines()
        .map(|line| {
            if line.contains("L") {
                -parse::<isize>(remove_prefix(line, "L"))
            } else {
                parse::<isize>(remove_prefix(line, "R"))
            }
        })
        .for_each(|num| {
            let num = num % 100;

            dial += num;

            if dial < 0 {
                dial += 100;
            } else if dial > 99 {
                dial -= 100;
            }

            if dial == 0 {
                times_0 += 1;
            }
        });

    times_0
}

pub fn solve_b(input: &str) -> usize {
    let mut dial = Dial(50);
    let mut times_0 = 0;

    input
        .lines()
        .map(|line| {
            if line.contains("L") {
                -parse::<isize>(remove_prefix(line, "L"))
            } else {
                parse::<isize>(remove_prefix(line, "R"))
            }
        })
        .for_each(|num| {
            times_0 += dial.rotate(num);
        });

    times_0
}

// I suck at math, so I just simulate the dial
struct Dial(isize);

impl Dial {
    fn rotate(
        &mut self,
        num: isize,
    ) -> usize {
        let mut times_0 = 0;

        if num > 0 {
            for _ in 0..num.abs() {
                if self.0 == 99 {
                    self.0 = 0;
                    times_0 += 1;
                } else {
                    self.0 += 1
                }
            }
        } else if num < 0 {
            for _ in 0..num.abs() {
                if self.0 == 0 {
                    self.0 = 99
                } else if self.0 == 1 {
                    self.0 -= 1;
                    times_0 += 1;
                } else {
                    self.0 -= 1;
                }
            }
        }

        times_0
    }
}
