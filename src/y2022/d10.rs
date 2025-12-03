use crate::aoc_lib::*;
use proc_macros::{from_regex, tile};
use Instruction::*;

pub fn solve_a(input: &str) -> isize {
    let mut instructions = input
        .lines()
        .map(Instruction::from_regex)
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

pub fn solve_b(input: &str) -> Board<Tile> {
    let mut instructions = input
        .lines()
        .map(Instruction::from_regex)
        .collect::<Vec<_>>();
    let mut board: Board<Tile> = Board::new(40, 6, || Tile::Dark);
    instructions.reverse();

    // cycle and x are zero based in this solution, as it makes things easier
    let mut cycle: isize = 0;
    let mut x = 0;
    let mut currently_added: Option<isize> = None;

    loop {
        if instructions.is_empty() {
            break board
        }

        if (x..=(x + 2)).contains(&(cycle % 40)) {
            board.set_tile_at_index(cycle as usize, Tile::Lit).unwrap();
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

#[derive(Clone, Copy, Debug)]
#[from_regex]
enum Instruction {
    #[reg(r#"noop"#)]
    Noop,
    #[reg(r#"addx (-*\d+)"#)]
    Add(isize)
}

#[tile]
pub enum Tile {
    #[t('#')]
    Lit,
    #[t('.')]
    Dark
}
