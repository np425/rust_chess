use std::fmt::{Debug, Error, Formatter};

use crate::player::Player::{Black, White};
use crate::square::Square;
use crate::square::Square::{Bishop, Empty, King, Knight, Pawn, Queen, Rook};

pub struct Board {
    pub squares: [Square; 64],
}

impl Board {
    fn resolve_index(&self, coord: Coord) -> usize {
        (coord.y * self.ranks_len() as isize + coord.x) as usize
    }

    pub fn get(&self, coord: Coord) -> Option<&Square> {
        self.squares.get(self.resolve_index(coord))
    }

    pub fn iter(&self) -> BoardIter {
        BoardIter {
            board: self,
            idx: 0,
        }
    }

    pub fn path_iter(&self, from: Coord, increment: Coord) -> BoardPathIter {
        BoardPathIter {
            board: self,
            current: from,
            increment,
        }
    }

    pub fn ranks_len(&self) -> usize {
        8
    }

    pub fn files_len(&self) -> usize {
        8
    }
}

pub struct BoardIter<'a> {
    board: &'a Board,
    idx: usize,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = (Coord, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        let square = self.board.squares.get(self.idx)?;

        let coord = Coord {
            x: (self.idx % self.board.ranks_len()) as isize,
            y: (self.idx / self.board.ranks_len()) as isize,
        };

        self.idx += 1;

        Some((coord, square))
    }
}

pub struct BoardPathIter<'a> {
    board: &'a Board,
    current: Coord,
    increment: Coord,
}

impl<'a> Iterator for BoardPathIter<'a> {
    type Item = (Coord, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.current, self.board.get(self.current)?);

        self.current.x += self.increment.x;
        self.current.y += self.increment.y;

        Some(result)
    }
}

impl Default for Board {
    fn default() -> Self {
        let squares = [
            Rook(White), Knight(White), Bishop(White), Queen(White), King(White), Bishop(White), Knight(White), Rook(White),
            Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White),
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black),
            Rook(Black), Knight(Black), Bishop(Black), Queen(Black), King(Black), Bishop(Black), Knight(Black), Rook(Black),
        ];

        Self { squares }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO: Show from the other way around
        let mut iter = self.iter();
        let Some((_, square)) = iter.next() else { return Err(Error) };
        write!(f, "{:?}", square)?;

        for (coord, square) in iter {
            if coord.x == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{:?}", square)?;
        };

        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy)]
pub struct Path {
    pub from: Coord,
    pub to: Coord,
}

impl Path {
    pub fn distance(&self) -> Coord {
        Coord {
            x: self.to.x - self.from.x,
            y: self.to.y - self.from.y
        }
    }
}