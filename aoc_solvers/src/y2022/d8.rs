use std::collections::HashSet;
use helpers::prelude::*;

pub fn solve_a(input: &str) -> usize {
    let board = Board::from_str_using_mapping(input, |c| c.to_digit(10).unwrap() as usize).unwrap();

    let mut visible = HashSet::new();

    board.rows().for_each(|row| add_visible(row, &mut visible));
    board.rows().for_each(|row| add_visible(row.rev(), &mut visible));
    board.columns().for_each(|column| add_visible(column, &mut visible));
    board.columns().for_each(|column| add_visible(column.rev(), &mut visible));

    visible.len()
}

fn add_visible<'a>(
    iter: impl Iterator<Item=(Position, &'a usize)>,
    visible: &mut HashSet<Position>
) {
    let mut highest = None;

    for (pos, value) in iter {
        match highest {
            Some(high) => if high < *value {
                highest = Some(usize::max(high, *value));
                visible.insert(pos);
            }
            None => {
                highest = Some(*value);
                visible.insert(pos);
            }
        }
    }
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::from_str_using_mapping(input, |c| c.to_digit(10).unwrap() as usize).unwrap();

    let mut max_score = 0;

    for pos in board.positions() {
        if board.pos_on_border(pos) {
            continue
        }

        let up = visible_trees_in_direction(&board, pos, YM);
        let down = visible_trees_in_direction(&board, pos, YP);
        let left = visible_trees_in_direction(&board, pos, XM);
        let right = visible_trees_in_direction(&board, pos, XP);

        max_score = usize::max(max_score, up * down * left * right)
    }

    max_score
}

fn visible_trees_in_direction(
    board: &Board<usize>,
    pos: Position,
    dir: Direction
) -> usize {
    let height = *board.get_tile(pos).unwrap();
    let mut amount = 0;

    // skip the first entry in the line, as it is the start itself
    for (_, value) in board.line_to_border(pos, dir).unwrap().skip(1) {
        amount += 1;

        if *value >= height {
            break
        }
    }

    amount
}
