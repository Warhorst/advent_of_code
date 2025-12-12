use crate::aoc_lib::*;

pub fn solve_a(input: &str) -> usize {
    // Just try to place all shapes using DFS with backtracking
    
    let problem = Problem::from(input);

    problem
        .regions
        .iter()
        .filter(|(dim, amounts)| {
            // Filter out all problems where the amount of all tiles would not fit the board
            let amount = amounts
                .iter()
                .enumerate()
                .map(|(i, amount)| amount * problem.shapes[i].len)
                .sum();
            let dim = dim.0 * dim.1;

            dim >= amount
        })
        .filter(|(dim, amounts)| solveable(*dim, amounts, &problem.shapes))
        .count()
}

fn solveable(
    dimension: Dimension,
    amounts: &[usize],
    shapes: &[Shape],
) -> bool {
    let mut board = Board::new(dimension.0, dimension.1, || false);
    let mut all_shapes = vec![];

    for (i, s) in shapes.iter().enumerate() {
        (0..amounts[i])
            .map(|_| s.clone())
            .for_each(|s_clone| all_shapes.push(s_clone));
    }

    solveable_(shapes, &mut board, 0)
}

fn solveable_(
    shapes: &[Shape],
    board: &mut Board<bool>,
    index: usize,
) -> bool {
    if index == shapes.len() {
        return true;
    }

    // println!("{index}");

    let shape = &shapes[index];

    for v in &shape.variants {
        for x in 0..board.width {
            for y in 0..board.height {
                if can_place(v, board, x, y) {
                    // println!("placable");
                    place(v, board, x, y);

                    if solveable_(shapes, board, index + 1) {
                        return true;
                    } else {
                        unplace(v, board, x, y);
                    }
                }
            }
        }
    }

    false
}

fn can_place(
    variant: &ShapeVariant,
    board: &Board<bool>,
    x: usize,
    y: usize,
) -> bool {
    variant.positions().all(|p| {
        if let Some(val) = board.get_tile(p!(p.x + x as isize, p.y + y as isize))
            && !*val
        {
            true
        } else {
            false
        }
    })
}

fn place(
    variant: &ShapeVariant,
    board: &mut Board<bool>,
    x: usize,
    y: usize,
) {
    variant.positions().for_each(|p| {
        board
            .set_tile(p!(p.x + x as isize, p.y + y as isize), true)
            .unwrap()
    });
}

fn unplace(
    variant: &ShapeVariant,
    board: &mut Board<bool>,
    x: usize,
    y: usize,
) {
    variant.positions().for_each(|p| {
        board
            .set_tile(p!(p.x + x as isize, p.y + y as isize), false)
            .unwrap()
    });
}

pub fn solve_b(_input: &str) -> usize {
    // No part B
    0
}

#[derive(Debug)]
struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<(Dimension, Vec<usize>)>,
}

impl From<&str> for Problem {
    fn from(value: &str) -> Self {
        let splits = value.split("\n\n").collect::<Vec<_>>();

        let mut shapes = vec![];

        for s in splits.iter().take(splits.len() - 1) {
            let shape = s
                .lines()
                .skip(1)
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| if c == '#' { Some(p!(x, y)) } else { None })
                })
                .collect::<Vec<_>>();
            shapes.push(Shape::new(shape));
        }

        let regions = splits
            .last()
            .unwrap()
            .lines()
            .map(|line| {
                let split = line.split_once(": ").unwrap();
                let dim_split = split.0.split_once("x").unwrap();
                let dimension = (parse(dim_split.0), parse(dim_split.1));
                let amounts = split.1.split(" ").map(parse).collect();
                (dimension, amounts)
            })
            .collect();

        Problem { shapes, regions }
    }
}

type Dimension = (usize, usize);

#[derive(Clone, Debug)]
struct Shape {
    len: usize,
    variants: Vec<ShapeVariant>,
}

impl Shape {
    fn new(mut positions: Vec<Position>) -> Self {
        let len = positions.len();
        let mut variants = vec![];

        let mut flip = positions
            .iter()
            .map(|pos| {
                if pos.x == 0 {
                    p!(2, pos.y)
                } else if pos.x == 2 {
                    p!(0, pos.y)
                } else {
                    *pos
                }
            })
            .collect::<Vec<_>>();

        // normal
        for _ in 0..4 {
            let variant = ShapeVariant::new(&positions);

            if !variants.contains(&variant) {
                variants.push(variant);
            }

            positions = Self::rotate(positions);
        }

        // flipped
        for _ in 0..4 {
            let variant = ShapeVariant::new(&flip);

            if !variants.contains(&variant) {
                variants.push(variant);
            }

            flip = Self::rotate(flip);
        }

        Shape { len, variants }
    }

    fn rotate(positions: Vec<Position>) -> Vec<Position> {
        // + 2 to fix now negative y values
        positions.into_iter().map(|p| p!(p.y, -p.x + 2)).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ShapeVariant([[bool; 3]; 3]);

impl ShapeVariant {
    fn new(positions: &[Position]) -> Self {
        let mut array = [[false; 3]; 3];

        for pos in positions {
            array[pos.x as usize][pos.y as usize] = true
        }

        ShapeVariant(array)
    }

    fn positions(&self) -> impl Iterator<Item = Position> {
        self.0.iter().enumerate().flat_map(|(x, arr)| {
            arr.iter()
                .enumerate()
                .filter_map(move |(y, val)| if *val { Some(p!(x, y)) } else { None })
        })
    }
}
