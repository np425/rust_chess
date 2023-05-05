use crate::board::{Board, Coord, Distance, Path};
use crate::player::Player;

pub struct Position {
    board: Board,
    king_pos: [Coord; 2],
    checks: Vec<Coord>,
    player: Player,
}

impl Position {
    fn update_checks(&self) -> Vec<Coord> {
        let king_pos = self.king_pos(self.player);
        let enemy = self.player.enemy();

        self.board.iter(Coord { x: 0, y: 0 }, Distance { x: 0, y: 0 })
            .filter_map(|(coord, square)| {
                let path = Path { from: coord, to: king_pos };
                (square.player() == Some(enemy) && square.can_attack(self, path)).then_some(coord)
            })
            .collect()
    }

    fn king_pos(&self, player: Player) -> Coord {
        match player {
            Player::White => self.king_pos[0],
            Player::Black => self.king_pos[1]
        }
    }

    // TODO: New
    pub fn new(board: Board, player: Player) {
        todo!()
    }

    pub fn checks(&self, player: Player) -> Option<&Vec<Coord>> {
        (player == self.player).then_some(&self.checks)
    }

    // TODO: Maybe extract it as a standalone function
    // TODO: Add constructors to path and coord
    // TODO: Add from into for path and coord
    // TODO: Simplify iterating over board (provide alternate methods)
    pub fn is_coord_defended(&self, target: Coord, by_player: Player) -> bool {
        self.board.iter(Coord { x: 0, y: 0 }, Distance { x: 1, y: 1 })
            .any(|(coord, square)| {
                let path = Path { from: coord, to: target };
                square.player() == Some(by_player) && square.can_attack(self, path)
            })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}

// TODO: Position Builder
