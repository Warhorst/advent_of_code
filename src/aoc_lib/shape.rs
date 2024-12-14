use std::collections::HashSet;
use pad::{p, Position, PositionPrinter};

/// A shape is a collection of positions which form some figure, image, etc.
/// The positions are all relative to the origin of the shape, which is top left
pub struct Shape {
    pub width: usize,
    pub height: usize,
    positions: HashSet<Position>
}

impl Shape {
    /// Create the shape from a string where the chars with letter 'X' are all considered part of
    /// the shape and every other char isn't.
    pub fn from_string(string: &str) -> Self {
        let width = string
            .lines()
            .next()
            .expect("the string should not be empty")
            .chars().count();
        let height = string.lines().count();
        let positions = string
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(move |(x, _)| p!(x, y))
            )
            .collect();

        Shape {
            width,
            height,
            positions
        }
    }

    pub fn positions(&self) -> impl IntoIterator<Item=Position> + '_ {
        self.positions.iter().copied()
    }

    pub fn print(&self) {
        PositionPrinter::new().draw_axis(true).print(self.positions.iter().copied())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::aoc_lib::shape::Shape;

    #[test]
    fn from_string_works() {
        let string = indoc! {"
            XXXXXXXX
            XX    XX
            XX    XX
            XXXXXXXX
        "};

        let shape = Shape::from_string(string);

        shape.print()
    }
}