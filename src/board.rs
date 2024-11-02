#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Square {
    #[default]
    Empty,
    Piece(Piece, Color),
}

impl Square {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
}

impl Board {
    pub fn square(&self, coord: Coord) -> Option<Square> {
        self.squares
            .get(coord.row as usize)?
            .get(coord.col as usize)
            .copied()
    }

    pub fn square_mut(&mut self, coord: Coord) -> Option<&mut Square> {
        self.squares
            .get_mut(coord.row as usize)?
            .get_mut(coord.col as usize)
    }

    pub fn move_piece(&mut self, from: Coord, to: Coord) -> Option<Square> {
        let from_square = self.square_mut(from)?;
        let from_copy = *from_square;
        *from_square = Square::Empty;

        let to_square = self.square_mut(to)?;
        let to_copy = *to_square;
        *to_square = from_copy;

        Some(to_copy)
    }
}

pub const STANDARD_BOARD: Board = Board {
    squares: [
        [
            Square::Piece(Piece::Rook, Color::White),
            Square::Piece(Piece::Knight, Color::White),
            Square::Piece(Piece::Bishop, Color::White),
            Square::Piece(Piece::Queen, Color::White),
            Square::Piece(Piece::King, Color::White),
            Square::Piece(Piece::Bishop, Color::White),
            Square::Piece(Piece::Knight, Color::White),
            Square::Piece(Piece::Rook, Color::White),
        ],
        [Square::Piece(Piece::Pawn, Color::White); 8],
        [Square::Empty; 8],
        [Square::Empty; 8],
        [Square::Empty; 8],
        [Square::Empty; 8],
        [Square::Piece(Piece::Pawn, Color::Black); 8],
        [
            Square::Piece(Piece::Rook, Color::Black),
            Square::Piece(Piece::Knight, Color::Black),
            Square::Piece(Piece::Bishop, Color::Black),
            Square::Piece(Piece::Queen, Color::Black),
            Square::Piece(Piece::King, Color::Black),
            Square::Piece(Piece::Bishop, Color::Black),
            Square::Piece(Piece::Knight, Color::Black),
            Square::Piece(Piece::Rook, Color::Black),
        ],
    ],
};
