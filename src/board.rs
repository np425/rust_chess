use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Player {
    #[default]
    White,
    Black,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Piece {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Queen,
    Rook,
    King,
}

// TODO: Refactor
pub type Square = Option<(Piece, Player)>;
pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, Default)]
pub struct Board {
    pub squares: [[Square; 8]; 8],
}

impl Index<Coord> for Board {
    type Output = Square;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.squares[index.0][index.1]
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.squares[index.0][index.1]
    }
}

impl Board {
    pub fn get(&self, coord: Coord) -> Option<Square> {
        self.squares
            .get(coord.0)
            .and_then(|file| file.get(coord.1).copied())
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Square> {
        self.squares
            .get_mut(coord.0)
            .and_then(|file| file.get_mut(coord.1))
    }
}
