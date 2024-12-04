use crate::aoc_lib::*;
use pad::p;

pub fn solve_a(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, p!(x, y))))
        .filter(|(c, _)| *c == 'X')
        .map(|(_, pos)| amount_xmas_at_position(&tiles, pos))
        .sum()
}

fn amount_xmas_at_position(tiles: &Vec<Vec<char>>, pos: Position) -> usize {
    // expects that the char at pos is an 'X', the caller must guarantee this

    let mut amount = 0;

    for dir in Direction::dirs() {
        if get_char_at_pos(tiles, pos) == Some('X')
            && get_char_at_pos(tiles, pos.position_in_direction(dir, 1)) == Some('M')
            && get_char_at_pos(tiles, pos.position_in_direction(dir, 2)) == Some('A')
            && get_char_at_pos(tiles, pos.position_in_direction(dir, 3)) == Some('S') {
            amount += 1
        }
    }

    amount
}

pub fn solve_b(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, p!(x, y))))
        .filter(|(c, _)| *c == 'A')
        .filter_map(|(_, pos)| is_x_mas_at_position(&tiles, pos))
        .count()
}

fn is_x_mas_at_position(tiles: &Vec<Vec<char>>, pos: Position) -> Option<()> {
    // I return Option<()> rather than bool just to make the code more elegant
    // expects that the char at pos is an 'A', the caller must guarantee this

    use Direction::*;

    let around = [
        get_char_at_pos(tiles, pos.position_in_direction(XMYP, 1))?,
        get_char_at_pos(tiles, pos.position_in_direction(XPYP, 1))?,
        get_char_at_pos(tiles, pos.position_in_direction(XPYM, 1))?,
        get_char_at_pos(tiles, pos.position_in_direction(XMYM, 1))?,
    ];

    if around == ['M', 'M', 'S', 'S']
        || around == ['S', 'M', 'M', 'S']
        || around == ['S', 'S', 'M', 'M']
        || around == ['M', 'S', 'S', 'M'] {
        Some(())
    } else {
        None
    }
}

fn get_char_at_pos(tiles: &Vec<Vec<char>>, pos: Position) -> Option<char> {
    tiles.get(pos.y as usize)?.get(pos.x as usize).copied()
}