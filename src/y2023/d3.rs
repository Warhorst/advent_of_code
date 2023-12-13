use std::collections::HashMap;
use pad::{p, Position};

pub fn solve_3a(input: &str) -> usize {
    let pos_char_map = create_pos_char_map(input);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut number_matches = vec![];
    let mut current_number = String::new();
    let mut a_symbol_is_connected = false;
    let mut prev_line = 0;

    for pos in p!(0, 0).iter_to(p!(width - 1, height - 1)) {
        // check if line changed
        if prev_line != pos.y {
            if !current_number.is_empty() && a_symbol_is_connected {
                number_matches.push(current_number.parse::<usize>().unwrap());
            }

            a_symbol_is_connected = false;
            current_number.clear();
            prev_line = pos.y
        }

        let c = match pos_char_map.get(&pos) {
            Some(c) => *c,
            None => continue
        };

        if !c.is_numeric() {
            if !current_number.is_empty() && a_symbol_is_connected {
                number_matches.push(current_number.parse::<usize>().unwrap());
            }

            a_symbol_is_connected = false;
            current_number.clear();

            continue;
        }

        current_number += format!("{c}").as_str();

        if !a_symbol_is_connected {
            a_symbol_is_connected = pos
                .neighbours()
                .into_iter()
                .flat_map(|p| pos_char_map.get(&p))
                .any(|c| !c.is_numeric() && *c != '.');
        }
    }

    number_matches.into_iter().sum()
}

pub fn solve_3b(input: &str) -> usize {
    let pos_char_map = create_pos_char_map(input);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut numbers_and_positions = vec![];
    let mut positions = vec![];
    let mut current_number = String::new();
    let mut prev_line = 0;

    for pos in p!(0, 0).iter_to(p!(width - 1, height - 1)) {
        // check if line changed
        if prev_line != pos.y {
            if !current_number.is_empty() {
                numbers_and_positions.push((current_number.parse::<usize>().unwrap(), positions.clone()));
            }

            positions.clear();
            current_number.clear();
            prev_line = pos.y
        }

        let c = match pos_char_map.get(&pos) {
            Some(c) => *c,
            None => continue
        };

        if !c.is_numeric() {
            if !current_number.is_empty() {
                numbers_and_positions.push((current_number.parse::<usize>().unwrap(), positions.clone()));
            }

            positions.clear();
            current_number.clear();

            continue;
        }

        positions.push(pos);
        current_number += format!("{c}").as_str();
    }

    pos_char_map
        .iter()
        .filter(|(_, c)| **c == '*')
        .map(|(pos, _)| pos)
        .filter_map(|pos| {
            let nums = numbers_and_positions
                .iter()
                .filter(|(_, positions)| pos.neighbours().into_iter().any(|pos| positions.contains(&pos)))
                .map(|(num, _)| *num)
                .collect::<Vec<_>>();

            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum()
}

fn create_pos_char_map(input: &str) -> HashMap<Position, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line
            .chars()
            .enumerate()
            .map(move |(j, char)| (p!(j, i), char))
        )
        .collect::<HashMap<_, _>>()
}
