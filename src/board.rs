use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Queen,
    Rook,
    King,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Coord {
    pub file: u8,
    pub rank: u8,
}

impl Coord {
    pub fn make(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        self.pieces.get(resolve_coord(coord)).copied()
    }

    pub fn square_unchecked_mut(&mut self, coord: Coord) -> &mut Square {
        &mut self.pieces[resolve_coord(coord)]
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

    pub fn path_iter(&self, coord: Coord, increment: (i8, i8)) -> BoardPathIter {
        BoardPathIter {
            board: self,
            coord,
            increment,
        }
    }
}

fn resolve_coord(coord: Coord) -> usize {
    (coord.file * 8 + coord.rank) as usize
}

fn next_coord(coord: Coord) -> Coord {
    Coord {
        file: coord.file + (coord.rank / 8),
        rank: (coord.rank + 1) % 8,
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

        self.coord = next_coord(current);

        Some((square, current))
    }
}

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
            file: self.coord.file.wrapping_add_signed(self.increment.0),
            rank: self.coord.rank.wrapping_add_signed(self.increment.1),
        };

        Some((square, current))
    }
}
