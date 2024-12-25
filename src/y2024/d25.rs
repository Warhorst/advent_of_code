use crate::aoc_lib::*;
use Tile::*;

pub fn solve_a(input: &str) -> usize {
    let mut locks = vec![];
    let mut keys = vec![];

    input
        .split("\n\n")
        .map(|block| Board::<Tile>::from_text(block))
        .for_each(|board| match board.rows().next().unwrap().all(|t| *t == Filled) {
            true => locks.push(Lock(column_counts(&board))),
            false => keys.push(Key(column_counts(&board)))
        });

    locks
        .into_iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, *key)))
        .filter(|(lock, key)| lock.fits_key(key))
        .count()
}

pub fn solve_b(_input: &str) -> usize {
    0
}

fn column_counts(board: &Board<Tile>) -> [usize; 5] {
    let mut iter = board
        .columns()
        .map(|col| col.filter(|t| **t == Filled).count() - 1);

    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap()
    ]
}

#[derive(Clone, Copy)]
struct Lock([usize; 5]);

impl Lock {
    fn fits_key(&self, key: &Key) -> bool {
        self.0.iter().enumerate().all(|(i, val)| val + key.0[i] <= 5)
    }
}

#[derive(Clone, Copy)]
struct Key([usize; 5]);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Filled,
    Empty
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Filled,
            '.' => Empty,
            _ => unreachable!()
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Filled => '#',
            Empty => '.'
        }
    }
}