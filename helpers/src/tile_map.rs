use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::prelude::*;

use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileMap<T> {
    pub width: usize,
    pub height: usize,
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

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
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
            .map(|y| (0..self.width)
                .map(|x| self.get(p!(x, y)))
                .map(Into::into)
                .join("")
            )
            .join("\n");

        writeln!(f, "{content}")
    }
}

impl <T> TileMap<T> where T: Copy {
    pub fn new(
        init: T,
        width: usize,
        height: usize
    ) -> Self {
        let tiles = (0..width)
            .flat_map(|x| (0..height)
                .map(move |y| (p!(x, y), init))
            )
            .collect();

        TileMap {
            width, height, tiles
        }
    }

    pub fn get(&self, pos: Position) -> T {
        *self.tiles.get(&pos).unwrap_or_else(|| panic!("The tile at position {pos:?} should exist"))
    }

    pub fn try_get(&self, pos: Position) -> Option<T> {
        self.tiles.get(&pos).copied()
    }

    pub fn rows(&self) -> impl IntoIterator<Item=Line<T>> + '_ {
        (0..self.height)
            .map(|y| Line::new((0..self.width)
                .map(|x| self.get(p!(x, y)))
                .collect())
            )
    }

    pub fn columns(&self) -> impl IntoIterator<Item=Line<T>> + '_ {
        (0..self.width)
            .map(|x| Line::new((0..self.height)
                .map(|y| self.get(p!(x, y)))
                .collect())
            )
    }

    pub fn iter(&self) -> impl Iterator<Item=(Position, T)> + '_{
        self.tiles.iter().map(|(pos, tile)| (*pos, *tile))
    }
}

impl <T> TileMap<T> {
    pub fn set(&mut self, pos: Position, tile: T) {
        if self.pos_in_bounds(pos) {
            self.tiles.insert(pos, tile);
        } else {
            panic!("outside of tile map!")
        }
    }

    /// Same as set, but does nothing if the position is not in bounds
    pub fn set_if_in_bounds(&mut self, pos: Position, tile: T) {
        if self.pos_in_bounds(pos) {
            self.tiles.insert(pos, tile);
        }
    }

    pub fn pos_in_bounds(&self, pos: Position) -> bool {
        0 <= pos.x && pos.x < self.width as isize && 0 <= pos.y && pos.y < self.height as isize
    }
}

#[cfg(test)]
mod tests {
    use Tile::*;

    use crate::prelude::TileMap;

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

    #[test]
    fn works() {
        let s  = "ABA\nBAC\nCAB";
        let _ = TileMap::<Tile>::from(s);
    }
}
