use crate::board::{Board, Coord, Piece, Player, Square};

#[derive(Debug, Clone, Copy, Default)]
pub enum CastlingSide {
    #[default]
    KingSide,
    QueenSide,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CastleOptions {
    pub king_side: bool,
    pub queen_side: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    board: Board,
    to_play: Player,
    castle_opts: CastleOptions,
}

// TODO: Next turn
impl Position {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn to_play(&self) -> Player {
        self.to_play
    }

    pub fn castle_options(&self) -> CastleOptions {
        self.castle_opts
    }

    pub fn can_move_piece(&self, origin: Coord, target: Coord) -> Option<()> {
        let origin_square = self.board.get(origin)?;
        let target_square = self.board.get(target)?;

        // Target square cannot be origin square
        if target_square == origin_square {
            return None;
        }

        // Origin square must be non empty
        let (origin_piece, origin_player) = origin_square?;

        // Origin piece has to be owned by player
        if origin_player != self.to_play {
            return None;
        }

        // Target square has to be empty or belong to enemy
        if matches!(target_square, Some((_, target_player)) if target_player != origin_player) {
            return None;
        }

        let (dy, dx) = (
            (target.0 - origin.0) as isize,
            (target.1 - target.1) as isize,
        );

        // TODO: Empty path, checks, en passant
        let can_move = match origin_piece {
            Piece::Rook => dx * dy == 0,
            Piece::King => dx.abs() <= 1 && dy.abs() <= 1,
            Piece::Queen => dx * dy == 0 || dx.abs() == dy.abs(),
            Piece::Knight => dx.abs() * dy.abs() == 2,
            Piece::Bishop => dx.abs() == dy.abs(),
            Piece::Pawn => {
                let direction = match origin_player {
                    Player::White => 1,
                    Player::Black => -1,
                };

                match target_square {
                    // Attack
                    Some(_) => dy == direction && dx.abs() == 1,

                    // Move normally
                    None => dy == direction && dx == 0,
                }
            }
        };

        if !can_move {
            return None;
        }

        return Some(());
    }

    pub fn can_castle(&self, side: CastlingSide) -> bool {
        let perm = match side {
            CastlingSide::QueenSide => self.castle_opts.queen_side,
            CastlingSide::KingSide => self.castle_opts.king_side,
        };

        if !perm {
            return false;
        }

        // TODO: Checks, empty path
        return true;
    }

    pub fn try_move_piece(&mut self, origin: Coord, target: Coord) -> Option<(Square, Square)> {
        self.can_move_piece(origin, target)?;

        let origin_square = self.board[origin];
        let target_square = self.board[target];

        // Move piece
        self.board[target] = self.board[origin];
        self.board[origin] = None;

        let first_row_y = match self.to_play {
            Player::White => 0,
            Player::Black => 7,
        };

        // Update castling permissions
        match origin_square {
            Some((Piece::Rook, _)) if origin == (first_row_y, 0) => {
                self.castle_opts.king_side = false
            }
            Some((Piece::Rook, _)) if origin == (first_row_y, 7) => {
                self.castle_opts.queen_side = false
            }
            Some((Piece::King, _)) => {
                self.castle_opts.king_side = false;
                self.castle_opts.queen_side = false;
            }
            _ => {}
        };

        Some((origin_square, target_square))
    }

    pub fn try_castle(&mut self, side: CastlingSide) -> bool {
        if !self.can_castle(side) {
            return false;
        }

        let first_row_y = match self.to_play {
            Player::White => 0,
            Player::Black => 7,
        };

        let (rook_x, direction): (usize, isize) = match side {
            CastlingSide::KingSide => (0, -1),
            CastlingSide::QueenSide => (7, 1),
        };

        let king_x = 4;

        let king_square = self.board[(first_row_y, king_x)];
        let rook_square = self.board[(first_row_y, rook_x)];

        // Remove pieces from old squares
        self.board[(first_row_y, king_x)] = None;
        self.board[(first_row_y, rook_x)] = None;

        // Place pieces to new squares
        self.board[(first_row_y, (king_x + direction * 2)] = king_square;
        self.board[(first_row_y, (king_x + direction)] = rook_square;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PositionBuilder {
    pub board: Board,
    pub to_play: Player,
}
