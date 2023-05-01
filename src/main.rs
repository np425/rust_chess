// TODO: Split into separate files

#[derive(Copy, Clone)]
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
    fn is_path_clear(&self, board: &Board, path: Path) -> bool {
        let d = path.diff();

        if d.x.abs() != d.y.abs() {
            return false;
        }
        if d.x == 0 {
            return true;
        }

        let incr = (
            if d.x > 0 { 1 } else { -1 },
            if d.y > 0 { 1 } else { -1 },
        );

        let iter = board.iter_path(path.from, incr);

        for square in iter.take_while(|(coord, _)| coord != path.to) {
            if let Some(Square::Empty) = square {
                return false;
            }
        }

        true
    }

    fn player(&self) -> Option<Player> {
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

    fn can_move(&self, pos: &Position, path: Path) -> bool {
        if !path.is_clear(&pos.board) {
            return false;
        }

        let d = path.diff();

        let player = self.player_ref()?;

        let no_checks = pos.checks(player)?.is_empty();

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

struct Position {
    board: Board,
    king_pos: [Coord; 2],
    checks: Vec<Coord>,
    player: Player,
}

impl Position {
    fn checks(&self, player: Player) -> Option<&Vec<Coord>> {
        (player == self.player).then_some(&self.checks)
    }

    // TODO: Maybe extract it as a standalone function
    fn is_coord_defended(&self, coord: Coord, by_player: Player) -> bool {
        for (target, square) in self.board.iter(Coord { x: 0, y: 0 }, (1, 1)) {
            let path = Path { from: coord, to: target };

            // TODO: can attack
            if *square.player_ref() == by_player && square.can_move(self, path) {
                return true;
            }
        }
        false
    }
}

struct Board {
    squares: [Square; 64],
}

impl Board {
    fn resolve_index(coord: Coord) -> usize {
        coord.y * 8 + coord.x
    }

    pub fn get(&self, coord: Coord) -> Option<&Square> {
        self.board.get(Self::resolve_index(coord))
    }

    pub fn iter(&self, from: Coord, incr: (isize, isize)) -> BoardPathIter {
        BoardPathIter {
            board: self,
            iter: from,
            incr,
        }
    }
}

struct BoardPathIter<'a> {
    board: &'a Board,
    iter: Coord,
    incr: (isize, isize),
}

impl<'a> Iterator for BoardPathIter<'a> {
    type Item = (Coord, &'a Square);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.iter, self.board.get(self.iter)?);

        self.iter.x += self.incr.0;
        self.iter.y += self.incr.1;

        Some(result)
    }
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Path {
    from: Coord,
    to: Coord,
}

impl Path {
    fn diff(&self) -> (isize, isize) {
        (self.to.x as isize - self.from.x, self.to.y as isize - self.from.y)
    }
}

// TODO: FEN, PGN, Game Impl
fn main() {
    println!("Hello, world!");
}
