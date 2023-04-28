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
    fn can_move(&self, board: &[Option<Piece>; 64], path: &Path) -> bool {
        let d = path.diff();

        match self {
            Piece::Rook(player) => d.x == 0 && d.y != 0 || d.x != 0 && d.y == 0,
            Piece::Knight(player) => {}
            Piece::Bishop(player) => {}
            Piece::Queen(player) => {}
            Piece::King(player) => {}
            Piece::Pawn(player) => {}
        }
    }
}

struct Position {
    board: [Option<Piece>; 64],
}

fn main() {
    println!("Hello, world!");
}
