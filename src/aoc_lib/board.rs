use pad::{p, Position, PositionIter};

/// A board of tiles where the tiles can be access by positions.
/// Orientation: The position (-inf, -inf) is top left and the position (+inf, +inf) is bottom right.
pub struct Board<T> {
    pub width: usize,
    pub height: usize,
    tiles: Vec<T>,
}

impl<T: From<char>> Board<T> {
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
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .expect("The input must contain at least one line")
            .chars()
            .count();

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
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn positions(&self) -> PositionIter {
        p!(0, 0).iter_to(p!(self.width - 1, self.height - 1))
    }

    pub fn get_tile(&self, pos: Position) -> Option<&T> {
        if !(0..self.width).contains(&(pos.x as usize)) || !(0..self.height).contains(&(pos.y as usize)) {
            None
        } else {
            self.tiles.get(pos.y as usize * self.width + pos.x as usize)
        }
    }
}