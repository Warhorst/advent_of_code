use proc_macros::from_regex;
use Instruction::*;

pub fn solve_a(input: &str) -> isize {
    let mut instructions = input
        .lines()
        .map(|line| Instruction::from_regex(line))
        .collect::<Vec<_>>();
    instructions.reverse();

    let mut cycle = 1;
    let mut x = 1;
    let mut cycles = vec![220, 180, 140, 100, 60, 20];
    let mut strength = 0;
    let mut currently_added = None;

    loop {
        if instructions.is_empty() {
            break strength
        }

        if !cycles.is_empty() && cycle == *cycles.last().unwrap() {
            strength += cycles.pop().unwrap() * x
        }

        cycle += 1;

        if currently_added.is_none() {
            let ins = instructions.pop().unwrap();

            match ins {
                Noop => {},
                Add(val) => currently_added = Some(val)
            }
        } else {
            x += currently_added.take().unwrap()
        }
    }
}

pub fn solve_b(_input: &str) -> usize {
    0
}

#[derive(Clone, Copy, Debug)]
#[from_regex]
enum Instruction {
    #[reg(r#"noop"#)]
    Noop,
    #[reg(r#"addx (-*\d+)"#)]
    Add(isize)
}