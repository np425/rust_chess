struct Coord {
    x: i8,
    y: i8,
}

struct Path {
    from: Coord,
    to: Coord,
}

impl Path {
    fn diff(&self) -> Coord {
        Coord {
            x: self.to.x - self.from.x,
            y: self.to.y - self.from.y,
        }
    }

    fn is_clear(&self, board: &[Square; 64]) -> bool {
        let d = self.diff();

        if d.x.abs() != d.y.abs() {
            return false;
        }
        if d.x == 0 {
            return true;
        }

        let incr = Coord {
            x: if d.x > 0 { 1 } else { -1 },
            y: if d.y > 0 { 1 } else { -1 },
        };

        let mut iter = Coord { x: path.from.x + incr.x, y: path.from.x + incr.y };

        while path.from.x != path.to.x && path.from.y != path.to.y {
            iter.x += incr.x;
            iter.y += incr.y;

            match board.get(iter.y * 8 + iter.x) {
                Some(Square::Empty) => true,
                _ => return false
            }
        }

        true
    }

}


enum Player {
    White,
    Black,
}

enum Square {
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    Rook(Player),
    King(Player),
    Pawn(Player),
    Empty
}

impl Square {
    fn can_move(&self, board: &[Square; 64], path: &Path) -> bool {
        let d = path.diff();
        if !path.is_clear(board) {
            return false
        }

        match self {
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
                // TODO: implement checks, etc
                true
            }
            Square::Pawn(player) => {
                let second_rank_y = match player {
                    Player::White => 1,
                    Player::Black => 6,
                };

                let dir = match player {
                    Player::White => 1,
                    Player::Black => -1,
                };

                d.y == 1 * dir || second_rank_y == path.from.y && d.y == 2 * dir
                // TODO: capturing
                // TODO: en passant
            }
            _ => false
        }
    }
}

// TODO: FEN, PGN, Game Impl

struct Position {
    board: [Square; 64],
}

fn main() {
    println!("Hello, world!");
}
