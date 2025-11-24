use pad::board::Board;
use pathfinding::prelude::astar;

pub fn solve_a(input: &str) -> usize {
    let board = Board::<Tile>::from(input);
    let start = board.get_positions_of(&Tile::Start).next().unwrap();
    let end = board.get_positions_of(&Tile::End).next().unwrap();

    let res = astar(
        &start,
        |pos| {
            let tile = board.get_tile(*pos).unwrap();
            pos.cardinal_neighbours()
                .into_iter()
                .filter(|p| board.pos_in_bounds(*p))
                .filter(|p| {
                    let t = board.get_tile(*p).unwrap();
                    t.height() < tile.height()
                        || t.height() - 1 == tile.height()
                        || t.height() == tile.height()
                })
                .map(|p| (p, 1))
        },
        |pos| pos.manhattan_distance(&end) as usize,
        |pos| *pos == end,
    )
    .expect("Shortest path should exist");

    res.1
}

pub fn solve_b(input: &str) -> usize {
    let board = Board::<Tile>::from(input);

    board
        .get_positions_of(&Tile::Step('a' as u8))
        .filter_map(|start| {
            let end = board.get_positions_of(&Tile::End).next().unwrap();

            let res = astar(
                &start,
                |pos| {
                    let tile = board.get_tile(*pos).unwrap();
                    pos.cardinal_neighbours()
                        .into_iter()
                        .filter(|p| board.pos_in_bounds(*p))
                        .filter(|p| {
                            let t = board.get_tile(*p).unwrap();
                            t.height() < tile.height()
                                || t.height() - 1 == tile.height()
                                || t.height() == tile.height()
                        })
                        .map(|p| (p, 1))
                },
                |pos| pos.manhattan_distance(&end) as usize,
                |pos| *pos == end,
            );

            match res {
                Some(r) => Some(r.1),
                None => None,
            }
        })
        .min()
        .unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Start,
    Step(u8),
    End,
}

impl Tile {
    fn height(&self) -> u8 {
        match self {
            Tile::Start => 'a' as u8 - 1,
            Tile::Step(v) => *v,
            Tile::End => 'z' as u8 + 1,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            'E' => Tile::End,
            c => Tile::Step(c as u8),
        }
    }
}
