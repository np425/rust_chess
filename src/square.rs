use std::fmt::{Debug, Formatter};

use crate::board::{Board, Coord, Path};
use crate::player::Player;
use crate::position::Position;

pub enum Square {
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    Rook(Player),
    King(Player),
    Pawn(Player),
    Empty,
}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.letter())
    }
}

impl Square {
    fn letter(&self) -> char {
        match self {
            Square::Knight(player) => match player {
                Player::White => 'N',
                _ => 'n'
            },
            Square::Bishop(player) => match player {
                Player::White => 'B',
                _ => 'b'
            },
            Square::Queen(player) => match player {
                Player::White => 'Q',
                _ => 'q'
            },
            Square::Rook(player) => match player {
                Player::White => 'R',
                _ => 'r'
            },
            Square::King(player) => match player {
                Player::White => 'K',
                _ => 'k'
            },
            Square::Pawn(player) => match player {
                Player::White => 'P',
                _ => 'p'
            },
            Square::Empty => '_'
        }
    }
    
    fn is_path_clear(board: &Board, path: Path) -> bool {
        let d = path.distance();

        if d.x.abs() != d.y.abs() {
            return false;
        }
        if d.x == 0 {
            return true;
        }

        let increment = Coord {
            x: d.x.signum(),
            y: d.y.signum(),
        };

        let iter = board.path_iter(path.from, increment);
        for (_, square) in iter.take_while(|(coord, _)| *coord != path.to) {
            if let Square::Empty = square {
                return false;
            }
        }

        true
    }

    pub fn player(&self) -> Option<Player> {
        match self {
            Square::Knight(player) => Some(*player),
            Square::Bishop(player) => Some(*player),
            Square::Queen(player) => Some(*player),
            Square::Rook(player) => Some(*player),
            Square::King(player) => Some(*player),
            Square::Pawn(player) => Some(*player),
            Square::Empty => None,
        }
    }

    pub fn can_move(&self, pos: &Position, path: Path) -> bool {
        self.defends(pos.board(), path)
    }

    pub fn can_attack(&self, pos: &Position, path: Path) -> bool {
        self.defends(pos.board(), path)
    }

    pub fn defends(&self, board: &Board, path: Path) -> bool {
        if !Self::is_path_clear(board, path) {
            return false;
        }

        let d = path.distance();

        match self {
            // TODO: defending with pieces
            Square::Rook(_) => {
                d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0
            }
            Square::Bishop(_) => {
                d.x.abs() == d.y.abs() && d.x != 0
            }
            Square::Queen(_) => {
                (d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0) || (d.x.abs() == d.y.abs() && d.x != 0)
            }
            Square::Knight(_) => {
                d.x.abs() * d.y.abs() == 2
            }
            Square::King(_) => {
                d.x.abs() <= 1 && d.y.abs() <= 1
            }
            Square::Pawn(_) => {
                let dir = match self.player() {
                    Some(Player::White) => 1,
                    Some(Player::Black) => -1,
                    _ => return false
                };

                d.y == dir && d.x == 1
            }
            _ => false
        }
    }
}
