use std::collections::HashMap;
use pad::{p, Position};
use Tile::*;

pub fn solve_13a(input: &str) -> usize {
    let blocks = input
        .split("\r\n\r\n")
        .map(Block::from)
        .collect::<Vec<_>>();

    // blocks.iter().for_each(|b| {
    //     println!("{:?}", b.find_vertical_mirror_line());
    //     println!("{:?}", b.find_horizontal_mirror_line());
    // });

    blocks
        .into_iter()
        .enumerate()
        .map(|(i, b)| match b.find_vertical_mirror_line() {
            Some(line) => line,
            None => {
                println!("{i}");
                b.find_horizontal_mirror_line() * 100
            }
        })
        .sum()
}

pub fn solve_13b(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Block {
    width: usize,
    height: usize,
    tile_map: HashMap<Position, Tile>,
}

impl Block {
    fn find_horizontal_mirror_line(&self) -> usize {
        let mut rows = vec![];

        for y in 0..self.height {
            let mut row = vec![];

            for x in 0..self.width {
                row.push(*self.tile_map.get(&p!(x, y)).unwrap())
            }

            rows.push((y, row))
        }

        let half = (self.height as f32 / 2.0).ceil() as usize;

        rows
            .windows(2)
            .filter(|window| window[0].1 == window[1].1)
            .filter(|window| {
                let take = self.height - window[1].0;
                if window[1].0 >= half {
                    let a = rows.iter().map(|row| &row.1).skip(self.height - take * 2).take(take).collect::<Vec<_>>();
                    let b = rows.iter().map(|row| &row.1).skip((self.height - take * 2) + 1).take(take).rev().collect::<Vec<_>>();

                    println!("top");
                    println!("a: {:?}", a);
                    println!("b: {:?}", b);

                    a == b
                } else {
                    let a = rows.iter().map(|row| &row.1).skip(0).take(take).collect::<Vec<_>>();
                    let b = rows.iter().map(|row| &row.1).skip(take).take(take).collect::<Vec<_>>();

                    println!("bottom");
                    println!("a: {:?}", a);
                    println!("b: {:?}", b);

                    a == b
                }
            })
            .map(|window| window[1].0)
            .next().unwrap()

        // rows
        //     .windows(4)
        //     .filter(|window| window[0].1 == window[3].1 && window[1].1 == window[2].1)
        //     .map(|window| window[2].0)
        //     .next().unwrap_or(self.height - 1)

        // unimplemented!()
    }

    fn find_vertical_mirror_line(&self) -> Option<usize> {
        let mut columns = vec![];

        for x in 0..self.width {
            let mut column = vec![];

            for y in 0..self.height {
                column.push(*self.tile_map.get(&p!(x, y)).unwrap())
            }

            columns.push((x, column))
        }

        let half = (self.width as f32 / 2.0).ceil() as usize;

        columns
            .windows(2)
            .filter(|window| window[0].1 == window[1].1)
            .filter(|window| {
                let take = self.width - window[1].0;

                if window[1].0 >= half {
                    let a = columns.iter().map(|col| &col.1).skip(self.width - take * 2).take(take).collect::<Vec<_>>();
                    let b = columns.iter().map(|col| &col.1).skip((self.width - take * 2) + 1).take(take).rev().collect::<Vec<_>>();

                    println!("right");
                    println!("a: {:?}", a);
                    println!("b: {:?}", b);

                    a == b
                } else {
                    let a = columns.iter().map(|col| &col.1).skip(0).take(take).collect::<Vec<_>>();
                    let b = columns.iter().map(|col| &col.1).skip(take).take(take).collect::<Vec<_>>();

                    println!("left");
                    println!("a: {:?}", a);
                    println!("b: {:?}", b);

                    a == b
                }
            })
            .map(|window| window[1].0)
            .next()

        // columns
        //     .windows(4)
        //     .filter(|window| window[0].1 == window[3].1 && window[1].1 == window[2].1)
        //     .map(|window| window[2].0)
        //     .next()

        // unimplemented!()
    }
}

impl From<&str> for Block {
    fn from(block_str: &str) -> Self {
        let mut tile_map = HashMap::new();

        block_str
            .lines()
            .enumerate()
            .for_each(|(y, line)| line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    tile_map.insert(p!(x, y), Tile::from(c));
                })
            );

        let width = block_str.lines().next().unwrap().len();
        let height = block_str.lines().count();

        Block {
            tile_map,
            width,
            height,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Ash,
            '#' => Rock,
            _ => panic!("invalid char")
        }
    }
}