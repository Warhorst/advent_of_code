use crate::aoc_lib::*;
use itertools::Itertools;

pub fn solve_a(input: &str) -> usize {
    use TileA::*;

    let mut split = input.split("\n\n");

    let mut board = Board::<TileA>::from_text(split.next().unwrap());
    let mut bot_pos = board.get_position_of(&Bot).unwrap();

    split
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars().map(map_to_dir))
        .for_each(|dir| {
            let pos_in_dir = bot_pos.position_in_direction(dir, 1);

            match board.get_tile(pos_in_dir).copied().unwrap() {
                // a box is the neighbour, check if it can be pushed
                Box => {
                    let mut step = 2;

                    loop {
                        let farther_away = bot_pos.position_in_direction(dir, step);

                        match board.get_tile(farther_away).copied().unwrap() {
                            // another box, check further
                            Box => step += 1,
                            // a wall is behind all the boxes, so it cant be pushed. break
                            Wall => { break }
                            // a free space is behind the boxes, shift everything in that direction
                            Free => {
                                for i in (1..=step).rev() {
                                    board.set_tile(
                                        bot_pos.position_in_direction(dir, i),
                                        board.get_tile(bot_pos.position_in_direction(dir, i - 1)).copied().unwrap(),
                                    );
                                    board.set_tile(
                                        bot_pos.position_in_direction(dir, i - 1),
                                        Free,
                                    );
                                }

                                bot_pos = pos_in_dir;

                                break;
                            }
                            // only one bot exists
                            _ => unreachable!()
                        }
                    }
                }
                // next to a wall, don't move
                Wall => {}
                // free, just move there and free the current position
                Free => {
                    board.set_tile(bot_pos, Free);
                    board.set_tile(pos_in_dir, Bot);
                    bot_pos = pos_in_dir;
                }
                // only one bot exists
                _ => unreachable!()
            }
        });

    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(t, p)| match *t == Box {
            true => Some(p),
            false => None
        })
        .map(|p| (p.x + p.y * 100) as usize)
        .sum()
}

pub fn solve_b(input: &str) -> usize {
    use TileB::*;

    let mut split = input.split("\n\n");
    let expanded_input = split
        .next()
        .unwrap()
        .lines()
        .map(|line|
            line
                .chars()
                .map(|c| match c {
                    '@' => "@.",
                    'O' => "[]",
                    '#' => "##",
                    '.' => "..",
                    _ => unreachable!()
                })
                .collect::<String>()
        )
        .join("\n");

    let mut board = Board::<TileB>::from_text(&expanded_input);
    let mut bot_pos = board.get_position_of(&Bot).unwrap();

    split
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars().map(map_to_dir))
        .for_each(|dir| {
            match dir {
                XP | XM => {
                    let pos_in_dir = bot_pos.position_in_direction(dir, 1);

                    match board.get_tile(pos_in_dir).copied().unwrap() {
                        // a box is the neighbour, check if it can be pushed
                        BoxL | BoxR => {
                            let mut step = 2;

                            loop {
                                let farther_away = bot_pos.position_in_direction(dir, step);

                                match board.get_tile(farther_away).copied().unwrap() {
                                    // another box, check further
                                    BoxL | BoxR => step += 1,
                                    // a wall is behind all the boxes, so it cant be pushed. break
                                    Wall => { break }
                                    // a free space is behind the boxes, shift everything in that direction
                                    Free => {
                                        for i in (1..=step).rev() {
                                            board.set_tile(
                                                bot_pos.position_in_direction(dir, i),
                                                board.get_tile(bot_pos.position_in_direction(dir, i - 1)).copied().unwrap(),
                                            );
                                            board.set_tile(
                                                bot_pos.position_in_direction(dir, i - 1),
                                                Free,
                                            );
                                        }

                                        bot_pos = pos_in_dir;

                                        break;
                                    }
                                    // only one bot exists
                                    _ => unreachable!()
                                }
                            }
                        }
                        // next to a wall, don't move
                        Wall => {}
                        // free, just move there and free the current position
                        Free => {
                            board.set_tile(bot_pos, Free);
                            board.set_tile(pos_in_dir, Bot);
                            bot_pos = pos_in_dir;
                        }
                        // only one bot exists
                        _ => unreachable!()
                    }
                }
                YP | YM => {
                    let pos_in_dir = bot_pos.position_in_direction(dir, 1);

                    match board.get_tile(pos_in_dir).copied().unwrap() {
                        // a box is the neighbour, check if it can be pushed
                        BoxL | BoxR => {
                            if adjacent_boxes_movable(pos_in_dir, dir, &board) {
                                let boxes = collect_adjacent_boxes(pos_in_dir, dir, &board);
                                //boxes.sort_by(|a, b| match dir {
                                //    YM => a.y.cmp(&b.y),
                                //    YP => b.y.cmp(&a.y),
                                //    _ => unreachable!()
                                //});

                                //println!("{:?}", boxes);

                                let boards_and_tiles = boxes
                                    .iter()
                                    .map(|pos| (pos, board.get_tile(*pos).copied().unwrap()))
                                    .collect::<Vec<_>>();

                                boards_and_tiles.iter().for_each(|(pos, _)| board.set_tile(**pos, Free));
                                boards_and_tiles.iter().for_each(|(pos, tile)| board.set_tile(pos.position_in_direction(dir, 1), *tile));

                                board.set_tile(bot_pos, Free);
                                board.set_tile(pos_in_dir, Bot);
                                bot_pos = pos_in_dir;
                            }
                        }
                        // next to a wall, don't move
                        Wall => {}
                        // free, just move there and free the current position
                        Free => {
                            board.set_tile(bot_pos, Free);
                            board.set_tile(pos_in_dir, Bot);
                            bot_pos = pos_in_dir;
                        }
                        // only one bot exists
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        });

    board
        .tiles_and_positions()
        .into_iter()
        .filter_map(|(t, p)| match *t == BoxL {
            true => Some(p),
            false => None
        })
        .map(|p| (p.x + p.y * 100) as usize)
        .sum()
}

fn adjacent_boxes_movable(
    box_position: Position,
    dir: Direction,
    board: &Board<TileB>,
) -> bool {
    use TileB::*;

    match board.get_tile(box_position).copied().unwrap() {
        BoxL => adjacent_boxes_movable(
            box_position.position_in_direction(dir, 1),
            dir,
            board,
        ) && adjacent_boxes_movable(
            (box_position + (1, 0)).position_in_direction(dir, 1),
            dir,
            board,
        ),
        BoxR => adjacent_boxes_movable(
            (box_position - (1, 0)).position_in_direction(dir, 1),
            dir,
            board,
        ) && adjacent_boxes_movable(
            box_position.position_in_direction(dir, 1),
            dir,
            board,
        ),
        Wall => false,
        Free => true,
        _ => unreachable!()
    }
}

fn collect_adjacent_boxes(
    box_position: Position,
    dir: Direction,
    board: &Board<TileB>,
) -> Vec<Position> {
    use TileB::*;

    match board.get_tile(box_position).copied().unwrap() {
        BoxL => {
            let mut boxes = vec![box_position, box_position + (1, 0)];

            boxes.extend(collect_adjacent_boxes(
                box_position.position_in_direction(dir, 1),
                dir,
                board,
            ));
            boxes.extend(collect_adjacent_boxes(
                (box_position + (1, 0)).position_in_direction(dir, 1),
                dir,
                board,
            ));

            boxes
        }
        BoxR => {
            let mut boxes = vec![box_position, box_position - (1, 0)];

            boxes.extend(collect_adjacent_boxes(
                (box_position - (1, 0)).position_in_direction(dir, 1),
                dir,
                board,
            ));
            boxes.extend(collect_adjacent_boxes(
                box_position.position_in_direction(dir, 1),
                dir,
                board,
            ));
            boxes
        }
        Free => vec![],
        _ => unreachable!()
    }
}

fn map_to_dir(value: char) -> Direction {
    match value {
        '^' => YM,
        'v' => YP,
        '<' => XM,
        '>' => XP,
        _ => unreachable!()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TileA {
    Bot,
    Box,
    Wall,
    Free,
}

impl From<char> for TileA {
    fn from(value: char) -> Self {
        use TileA::*;

        match value {
            '@' => Bot,
            'O' => Box,
            '#' => Wall,
            '.' => Free,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TileB {
    Bot,
    BoxL,
    BoxR,
    Wall,
    Free,
}

impl From<char> for TileB {
    fn from(value: char) -> Self {
        use TileB::*;

        match value {
            '@' => Bot,
            '[' => BoxL,
            ']' => BoxR,
            '#' => Wall,
            '.' => Free,
            _ => unreachable!()
        }
    }
}

impl Into<char> for TileB {
    fn into(self) -> char {
        use TileB::*;

        match self {
            Bot => '@',
            BoxL => '[',
            BoxR => ']',
            Wall => '#',
            Free => '.',
        }
    }
}