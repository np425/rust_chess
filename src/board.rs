#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

// -------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Queen,
    Rook,
    King,
}

// -------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    #[default]
    Empty,
    Piece(Piece, Color),
}

impl Square {
    pub fn piece(self) -> Option<(Piece, Color)> {
        match self {
            Self::Empty => None,
            Self::Piece(piece, color) => Some((piece, color)),
        }
    }

    pub fn piece_kind(self) -> Option<Piece> {
        match self {
            Self::Empty => None,
            Self::Piece(piece, _) => Some(piece),
        }
    }

    pub fn player(self) -> Option<Color> {
        match self {
            Self::Empty => None,
            Self::Piece(_, player) => Some(player),
        }
    }
}

// -------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Coord {
    pub row: u8,
    pub col: u8,
}

impl Coord {
    pub fn make(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}

// -------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    pieces: [Square; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [Square::default(); 64],
        }
    }
}

impl Board {
    pub fn square(&self, coord: Coord) -> Option<Square> {
        self.pieces.get(Self::resolve_index(coord)).copied()
    }

    pub fn square_mut(&mut self, coord: Coord) -> Option<&mut Square> {
        self.pieces.get_mut(Self::resolve_index(coord))
    }

    pub fn square_unchecked(&self, coord: Coord) -> Square {
        self.pieces[Self::resolve_index(coord)]
    }

    pub fn square_unchecked_mut(&mut self, coord: Coord) -> &mut Square {
        &mut self.pieces[Self::resolve_index(coord)]
    }

    pub fn move_unchecked(&mut self, origin_coord: Coord, target_coord: Coord) -> (Square, Square) {
        let origin = self.square_unchecked_mut(origin_coord);
        let origin_square = *origin;
        *origin = Square::Empty;

        let target = self.square_unchecked_mut(target_coord);
        let target_square = *target;
        *target = origin_square;

        (origin_square, target_square)
    }

    pub fn iter(&self, origin_coord: Coord) -> impl Iterator<Item = (Square, Coord)> + '_ {
        BoardIter {
            board: self,
            coord: origin_coord,
        }
    }

    pub fn iter_path(
        &self,
        origin_coord: Coord,
        increment: (i8, i8),
    ) -> impl Iterator<Item = (Square, Coord)> + '_ {
        BoardPathIter {
            board: self,
            coord: origin_coord,
            increment,
        }
    }

    fn resolve_index(coord: Coord) -> usize {
        (coord.row * 8 + coord.col) as usize
    }
}

// -------------

pub struct BoardIter<'a> {
    board: &'a Board,
    coord: Coord,
}

impl Iterator for BoardIter<'_> {
    type Item = (Square, Coord);
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.coord;
        let square = self.board.square(current)?;

        self.coord = Coord {
            row: current.row + current.col / 8,
            col: (current.col + 1) % 8,
        };

        Some((square, current))
    }
}

// -------------

pub struct BoardPathIter<'a> {
    board: &'a Board,
    coord: Coord,
    increment: (i8, i8),
}

impl Iterator for BoardPathIter<'_> {
    type Item = (Square, Coord);
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.coord;
        let square = self.board.square(current)?;

        self.coord = Coord {
            row: current.row.wrapping_add_signed(self.increment.0),
            col: current.col.wrapping_add_signed(self.increment.1),
        };

        Some((square, current))
    }
}
