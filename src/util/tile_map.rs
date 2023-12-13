use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use pad::{p, Position};

#[derive(Clone, Debug)]
pub struct TileMap<T> {
    width: usize,
    height: usize,
    tiles: HashMap<Position, T>
}

/// A horizontal or vertical line from a TileMap
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line<T> {
    values: Vec<T>
}

impl <T> Line<T> {
    pub fn new(values: Vec<T>) -> Self {
        Line {
            values
        }
    }
}

impl <T> From<&str> for TileMap<T> where T: From<char> {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .map(T::from)
                .enumerate()
                .map(move |(x, tile)| (p!(x, y), tile))
            )
            .collect();

        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        TileMap {
            width, height, tiles
        }
    }
}

impl <T> Display for TileMap<T> where T: Copy + Into<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = (0..self.height)
            .into_iter()
            .map(|y| (0..self.width)
                .into_iter()
                .map(|x| self.get(p!(x, y)))
                .map(Into::into)
                .join(" ")
            )
            .join("\n");

        writeln!(f, "{content}")
    }
}

impl <T> TileMap<T> where T: Copy {
    pub fn get(&self, pos: Position) -> T {
        *self.tiles.get(&pos).expect(format!("The tile at position {:?} should exist", pos).as_str())
    }

    pub fn rows(&self) -> impl IntoIterator<Item=Line<T>> + '_ {
        (0..self.height)
            .into_iter()
            .map(|y| Line::new((0..self.width)
                .into_iter()
                .map(|x| self.get(p!(x, y)))
                .collect())
            )
    }

    pub fn columns(&self) -> impl IntoIterator<Item=Line<T>> + '_ {
        (0..self.width)
            .into_iter()
            .map(|x| Line::new((0..self.height)
                .into_iter()
                .map(|y| self.get(p!(x, y)))
                .collect())
            )
    }
}

#[cfg(test)]
mod tests {
    use Tile::*;

    use crate::util::TileMap;

    #[derive(Copy, Clone)]
    enum Tile {
        A,
        B,
        C
    }

    impl From<char> for Tile {
        fn from(c: char) -> Self {
            match c {
                'A' => A,
                'B' => B,
                'C' => C,
                _ => panic!("invalid char")
            }
        }
    }

    impl Into<char> for Tile {
        fn into(self) -> char {
            match self {
                A => 'A',
                B => 'B',
                C => 'C'
            }
        }
    }

    #[test]
    fn works() {
        let s  = "ABA\nBAC\nCAB";
        let board = TileMap::<Tile>::from(s);
        println!("{board}")
    }
}