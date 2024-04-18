pub mod board;
pub mod position;

use board::*;
use position::*;

fn incr_char(chr1: char, incr: u8) -> char {
    ((chr1 as u8) + incr) as char
}

fn display_board(board: &Board, _perspective: Color) {
    print!(" | ");
    for row in 0..8 {
        print!("{} ", incr_char('A', row))
    }
    print!("| ");
    println!();
    println!("---------------------");

    // reversed to put white at bottom
    for row in (0..8).rev() {
        print!("{}| ", row + 1);

        for col in 0..8 {
            let square = board.square_unchecked(Coord::make(row, col));

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

            print!("{} ", chr);
        }
        print!("|{}", row + 1);

        println!();
    }

    println!("---------------------");
    print!(" | ");
    for row in 0..8 {
        print!("{} ", incr_char('A', row))
    }
    print!("| ");
    println!();
}

fn main() {
    let pos = Position::default();
    display_board(pos.board(), pos.to_play());
}
