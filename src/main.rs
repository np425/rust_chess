pub mod board;
pub mod position;

use board::*;
use position::*;

fn display_board(board: &Board) {
    let iter = board.iter(Coord::default());

    for _row in 0..8 {
        for (square, _) in iter {
            let mut chr = match square.piece_kind() {
                None => ' ',
                Some(Piece::Pawn) => 'p',
                Some(Piece::Rook) => 'r',
                Some(Piece::King) => 'k',
                Some(Piece::Queen) => 'q',
                Some(Piece::Knight) => 'n',
                Some(Piece::Bishop) => 'b',
            };

            if square.player() == Some(Color::White) {
                chr = chr.to_ascii_uppercase();
            }

            print!("{}", chr);
        }

        println!();
    }
}

fn main() {
    let pos = Position::default();
    display_board(pos.board());

    unimplemented!();
}
