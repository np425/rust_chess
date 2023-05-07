use crate::board::{Board, Coord, Path};
use crate::player::Player;
use crate::square::Square;

// TODO: Pins, Discovered checks
pub struct Position {
    board: Board,
    king_pos: [Coord; 2],
    checks: Vec<Coord>,
    player: Player,
}

impl Position {
    pub fn king_pos(&self, player: Player) -> Coord {
        match player {
            Player::White => self.king_pos[0],
            Player::Black => self.king_pos[1]
        }
    }

    pub fn checks(&self, player: Player) -> Option<&Vec<Coord>> {
        (player == self.player).then_some(&self.checks)
    }

    pub fn is_coord_defended(&self, target: Coord, by_player: Player) -> bool {
        self.board.iter()
            .any(|(coord, square)| {
                let path = Path { from: coord, to: target };
                square.player() == Some(by_player) && square.can_attack(self, path)
            })
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn make_move(&mut self) -> bool {
        true
    }
}

pub struct PositionBuilder {
    board: Board,
    player: Player,
    king_pos: [Vec<Coord>; 2],
    checks: [Vec<Coord>; 2],
}

impl PositionBuilder {
    pub fn new(board: Board, player: Player) -> Self {
        let mut builder = Self {
            board,
            player,
            king_pos: [vec![], vec![]],
            checks: [vec![], vec![]],
        };

        builder.find_kings();
        builder.find_checks();

        builder
    }

    fn find_kings(&mut self) {
        self.board.iter()
            .for_each(|(coord, square)| {
                let Square::King(player) = square else { return; };

                match player {
                    Player::White => self.king_pos[0].push(coord),
                    Player::Black => self.king_pos[1].push(coord),
                }
            });
    }

    fn find_checks(&mut self) {
        for (coord, square) in self.board.iter() {
            for king_coord in self.king_pos.iter().flatten() {
                let Some(king) = self.board.get(*king_coord) else { continue };
                let path = Path { from: coord, to: *king_coord };

                if king.player() != square.player() && square.defends(&self.board, path) {
                    match king.player() {
                        Some(Player::White) => self.checks[0].push(coord),
                        Some(Player::Black) => self.checks[1].push(coord),
                        _ => {}
                    }
                }
            }
        }
    }

    fn validate(&self) -> bool {
        todo!()
    }

    pub fn try_build(self) -> Option<Position> {
        let [checkA, checkB] = self.checks;

        let checks = match self.player {
            Player::White => checkA,
            Player::Black => checkB
        };

        let king_pos = [*self.king_pos[0].get(0)?, *self.king_pos[1].get(0)?];

        Some(Position {
            board: self.board,
            king_pos,
            checks,
            player: self.player,
        })
    }
}