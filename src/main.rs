#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
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

    // TODO: take position, to not hardcore board size
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

impl Player {
    fn enemy(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
}

enum Square {
    Knight(Player),
    Bishop(Player),
    Queen(Player),
    Rook(Player),
    King(Player),
    Pawn(Player),
    Empty,
}

impl Square {
    fn player_ref(&self) -> Option<&Player> {
        match self {
            Square::Knight(player) => Some(player),
            Square::Bishop(player) => Some(player),
            Square::Queen(player) => Some(player),
            Square::Rook(player) => Some(player),
            Square::King(player) => Some(player),
            Square::Pawn(player) => Some(player),
            Square::Empty => None,
        }
    }

    fn can_move(&self, pos: &Position, path: &Path) -> bool {
        if !path.is_clear(&pos.board) {
            return false;
        }

        let d = path.diff();

        // TODO: checks for current player
        let no_checks = pos.checks.is_empty();

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
                d.x.abs() <= 1 && d.y.abs() <= 1
            }
            Square::Pawn(_) => {
                let second_rank_y = match player {
                    Player::White => 1,
                    Player::Black => pos.board.len() / 8 - 2,
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
    king_pos: [Coord; 2],
    checks: Vec<Coord>,
    player: Player,
}

impl Position {
    fn checks(&self, player: &Player) -> Option<&Vec<Coord>> {
        (player == self.player).then_some(&self.checks)
    }

    fn is_square_defended(&self, coord: &Coord, by_player: Player) -> bool {
        for (idx, square) in self.board.iter().enumerate() {
            // TODO: refactor
            // TODO: can attack
            if *square.player_ref() == by_player && square.can_move(self, &Path { from: Coord { x: idx % 8, y: idx / 8 }, to: *coord }) {
                return false;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
