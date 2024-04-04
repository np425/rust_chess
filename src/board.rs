#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Queen,
    Rook,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Coord {
    coord: u8,
}

impl Coord {
    pub fn make(rank: u8, file: u8) -> Self {
        Self {
            coord: file * 8 + rank,
        }
    }

    pub fn file(&self) -> u8 {
        self.coord / 8
    }

    pub fn rank(&self) -> u8 {
        self.coord % 8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pieces: [Square; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [Square::Empty; 64],
        }
    }
}

impl Board {
    pub fn square(&self, coord: Coord) -> Option<Square> {
        self.pieces.get(coord.coord as usize).copied()
    }

    pub fn square_unchecked_mut(&mut self, coord: Coord) -> &mut Square {
        &mut self.pieces[coord.coord as usize]
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

    pub fn iter(&self) -> BoardIter {
        BoardIter {
            board: self,
            coord: Coord::default(),
        }
    }
}

pub struct BoardIter<'a> {
    board: &'a Board,
    coord: Coord,
}

impl Iterator for BoardIter<'_> {
    type Item = (Square, Coord);
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.coord;
        let square = self.board.square(current)?;

        self.coord.coord += 1;
        Some((square, current))
    }
}
