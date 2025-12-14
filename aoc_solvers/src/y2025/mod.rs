use crate::{Input, PuzzleOutput, PuzzleResult, RunConfig};

pub mod d1;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

pub(super) fn solve(
    day: u8,
    run_config: RunConfig,
) -> Option<PuzzleResult> {
    Some(match day {
        1 => super::solve_puzzle(run_config, input(day), d1::solve_a, d1::solve_b),
        2 => super::solve_puzzle(run_config, input(day), d2::solve_a, d2::solve_b),
        3 => super::solve_puzzle(run_config, input(day), d3::solve_a, d3::solve_b),
        4 => super::solve_puzzle(run_config, input(day), d4::solve_a, d4::solve_b),
        5 => super::solve_puzzle(run_config, input(day), d5::solve_a, d5::solve_b),
        6 => super::solve_puzzle(run_config, input(day), d6::solve_a, d6::solve_b),
        7 => super::solve_puzzle(run_config, input(day), d7::solve_a, d7::solve_b),
        8 => super::solve_puzzle(run_config, input(day), d8::solve_a, d8::solve_b),
        9 => super::solve_puzzle(run_config, input(day), d9::solve_a, d9::solve_b),
        10 => super::solve_puzzle(run_config, input(day), d10::solve_a, d10::solve_b),
        11 => super::solve_puzzle(run_config, input(day), d11::solve_a, d11::solve_b),
        12 => super::solve_puzzle(run_config, input(day), d12::solve_a, d12::solve_b),
        _ => return None,
    })
}

fn input<A: PuzzleOutput, B: PuzzleOutput>(day: u8) -> Input<A, B> {
    Input::<A, B>::load(day, 2025)
}
