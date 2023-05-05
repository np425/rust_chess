use crate::board::{Board, Distance, Path};
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

impl Square {
    fn is_path_clear(board: &Board, path: Path) -> bool {
        let d = path.distance();

        if d.x.abs() != d.y.abs() {
            return false;
        }
        if d.x == 0 {
            return true;
        }

        let incr = Distance {
            x: if d.x > 0 { 1 } else { -1 },
            y: if d.y > 0 { 1 } else { -1 },
        };

        let iter = board.iter(path.from, incr);

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
        if !Self::is_path_clear(pos.board(), path) {
            return false;
        }

        let d = path.distance();

        let player = self.player().unwrap();

        let no_checks = pos.checks(player)
            .map_or(false, |checks| checks.is_empty());

        match self {
            // TODO: defending with pieces
            Square::Rook(_) => {
                no_checks && d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0
            }
            Square::Bishop(_) => {
                no_checks && d.x.abs() == d.y.abs() && d.x != 0
            }
            Square::Queen(_) => {
                no_checks && (d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0) || (d.x.abs() == d.y.abs() && d.x != 0)
            }
            Square::Knight(_) => {
                no_checks && d.x.abs() * d.y.abs() == 2
            }
            Square::King(_) => {
                // TODO: implement checks, etc
                d.x.abs() <= 1 && d.y.abs() <= 1 && !pos.is_coord_defended(path.to, player.enemy())
            }
            Square::Pawn(_) => {
                let second_rank_y = match player {
                    Player::White => 1,
                    Player::Black => pos.board().ranks_len() - 2,
                };

                let dir = match player {
                    Player::White => 1,
                    Player::Black => -1,
                };

                d.y == dir || second_rank_y == path.from.y && d.y == 2 * dir
            }
            _ => false
        }
    }

    pub fn can_attack(&self, pos: &Position, path: Path) -> bool {
        if !Self::is_path_clear(pos.board(), path) {
            return false;
        }

        let d = path.distance();

        let player = self.player().unwrap();

        let no_checks = pos.checks(player)
            .map_or(false, |checks| checks.is_empty());

        match self {
            // TODO: defending with pieces
            Square::Rook(_) => {
                no_checks && d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0
            }
            Square::Bishop(_) => {
                no_checks && d.x.abs() == d.y.abs() && d.x != 0
            }
            Square::Queen(_) => {
                no_checks && (d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0) || (d.x.abs() == d.y.abs() && d.x != 0)
            }
            Square::Knight(_) => {
                no_checks && d.x.abs() * d.y.abs() == 2
            }
            Square::King(_) => {
                // TODO: implement checks, etc
                d.x.abs() <= 1 && d.y.abs() <= 1 && !pos.is_coord_defended(path.to, player.enemy())
            }
            Square::Pawn(_) => {
                let dir = match player {
                    Player::White => 1,
                    Player::Black => -1,
                };

                d.y == dir && d.x == 1
                // TODO: en passant
            }
            _ => false
        }
    }
}
