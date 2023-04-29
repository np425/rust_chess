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
}

enum Player {
    White,
    Black,
}

enum Piece {
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    Rook(Player),
    King(Player),
    Pawn(Player),
}

impl Piece {
    fn is_path_clear(board: &[Option<Piece>; 64], path: &Path) -> bool {
        let d = path.diff();

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
                Some(None) => true,
                _ => return false
            }
        }

        true
    }

    fn can_move(&self, board: &[Option<Piece>; 64], path: &Path) -> bool {
        let d = path.diff();
        if !Self::is_path_clear(board, path) {
            return false
        }

        match self {
            Piece::Rook(_) => {
                d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0
            }
            Piece::Bishop(_) => {
                d.x.abs() == d.y.abs() && d.x != 0
            }
            Piece::Queen(_) => {
                (d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0) || (d.x.abs() == d.y.abs() && d.x != 0)
            }
            Piece::Knight(_) => {
                d.x.abs() * d.y.abs() == 2
            }
            Piece::King(_) => {
                // TODO: implement checks, etc
                true
            }
            Piece::Pawn(player) => {
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
        }
    }
}

// TODO: FEN, PGN, Game Impl

struct Position {
    board: [Option<Piece>; 64],
}

fn main() {
    println!("Hello, world!");
}
