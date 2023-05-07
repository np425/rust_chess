use crate::board::Board;
use crate::player::Player;
use crate::position::PositionBuilder;

mod player;
mod square;
mod board;
mod position;

// TODO: FEN, PGN, Game Impl
fn main() {
    let board = Board::default();
    let builder = PositionBuilder::new(board, Player::White);
    let Some(position) = builder.try_build() else {
        println!("Failed to build position!");
        return;
    };
    println!("{:?}", position.board());
}
