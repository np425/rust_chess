use crate::board::{Board, Coord, Distance, Path};
use crate::player::Player;

pub struct Position {
    board: Board,
    king_pos: [Coord; 2],
    checks: Vec<Coord>,
    player: Player,
}

impl Position {
    pub fn checks(&self, player: Player) -> Option<&Vec<Coord>> {
        (player == self.player).then_some(&self.checks)
    }

    // TODO: Maybe extract it as a standalone function
    pub fn is_coord_defended(&self, coord: Coord, by_player: Player) -> bool {
        for (target, square) in self.board.iter(Coord { x: 0, y: 0 }, Distance { x: 1, y: 1 }) {
            let path = Path { from: coord, to: target };

            // TODO: can attack
            if let Some(player) = square.player() {
                if player == by_player && square.can_move(self, path) {
                    return true;
                }
            }
        }
        false
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}

