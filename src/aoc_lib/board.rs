use pad::{p, Position, PositionIter};

/// A board of tiles where the tiles can be access by positions.
/// Orientation: The position (-inf, -inf) is top left and the position (+inf, +inf) is bottom right.
pub struct Board<T> {
    pub width: usize,
    pub height: usize,
    tiles: Vec<T>,
}

impl<T: From<char>> Board<T> {
    /// Create the board from a text input, where each character represents
    /// a designated board element. If the input might contain special elements,
    /// use Board::board_and_specials_from_text instead
    /// * `input` - The text input which represents the board. Expected to be a multiline string
    ///             where every line has the same amount of characters
    pub fn from_text(input: &str) -> Self {
        let width = width_from_input(input);
        let height = height_from_input(input);

        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(|c| T::from(c)))
            .collect();

        Board {
            width,
            height,
            tiles,
        }
    }

    /// Create a board and all special tiles from the given text input
    ///
    /// * `input` - The text input which represents the board. Expected to be a multiline string
    ///             where every line has the same amount of characters
    /// * `special_map` - A mapping closure which might map a given char and position to a special
    ///                   Tile and a Tile default. The special tile is part of the input, but not
    ///                   of the actual board, so it gets extracted. If the char at the current
    ///                   position could be mapped to a special tile, the special case gets stored
    ///                   and the spot on the board gets replaced by the default tile, specified by
    ///                   the mapper.
    pub fn board_and_specials_from_text<S>(
        input: &str,
        special_map: impl Fn(char, Position) -> Option<(S, T)>,
    ) -> (Self, Vec<S>) {
        let width = width_from_input(input);
        let height = height_from_input(input);

        let mut tiles = Vec::with_capacity(width * height);
        let mut specials = Vec::new();

        input
            .lines()
            .enumerate()
            .for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)| match special_map(c, p!(x, y)) {
                Some((special, tile)) => {
                    specials.push(special);
                    tiles.push(tile)
                }
                None => tiles.push(T::from(c))
            }));

        (
            Board {
                width,
                height,
                tiles,
            },
            specials
        )
    }
}

impl<T> Board<T> {
    /// Same as Board::from_text, but the tile type does not implement From<char>, so the provided
    /// mapper is used to parse the chars to tiles.
    /// * `input` - The text input which represents the board. Expected to be a multiline string
    ///             where every line has the same amount of characters
    /// * `map` - Mapper which converts a char to at tile
    pub fn from_text_and_mapping(input: &str, map: impl Fn(char) -> T) -> Self {
        let width = width_from_input(input);
        let height = height_from_input(input);

        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(|c| map(c)))
            .collect();

        Board {
            width,
            height,
            tiles,
        }
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn pos_in_bounds(&self, pos: Position) -> bool {
        (0..self.width).contains(&(pos.x as usize)) && (0..self.height).contains(&(pos.y as usize))
    }

    pub fn tiles_and_positions(&self) -> impl IntoIterator<Item=(&T, Position)> {
        self.positions()
            .into_iter()
            .map(|pos| (self.get_tile(pos).expect("the tile must exist"), pos))
    }

    pub fn positions(&self) -> PositionIter {
        p!(0, 0).iter_to(p!(self.width - 1, self.height - 1))
    }

    pub fn get_tile(&self, pos: Position) -> Option<&T> {
        if !self.pos_in_bounds(pos) {
            None
        } else {
            self.tiles.get(pos.y as usize * self.width + pos.x as usize)
        }
    }

    pub fn get_tiles(&self, positions: impl IntoIterator<Item=Position>) -> impl IntoIterator<Item=&T> {
        positions
            .into_iter()
            .flat_map(|pos| self.get_tile(pos))
    }
}

fn width_from_input(input: &str) -> usize {
    input
        .lines()
        .next()
        .expect("The input must contain at least one line")
        .chars()
        .count()
}

fn height_from_input(input: &str) -> usize {
    input.lines().count()
}