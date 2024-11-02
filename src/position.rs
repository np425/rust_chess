use crate::board::{Board, Color, Coord, Piece, Square, STANDARD_BOARD};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastleSide {
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastleRights {
    pub king: bool,
    pub queen: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Playing,
    Checkmate(Color),
    Stalemate(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveErr {
    PieceNotOwned,
    DestinationOccupied,
    KingInCheck,
    NoCastlingRight,
    PathBlocked,
    InvalidPromotion,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MoveInfo {
    from: (Coord, Square),
    to: (Coord, Square),
    captures: Option<Piece>,
    promotion: Option<Piece>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    board: Board,
    to_play: Color,
    castle_rights: (CastleRights, CastleRights),

    state: State,

    checks: Vec<Coord>,
    king_coord: (Coord, Coord),
}

impl Default for Position {
    fn default() -> Self {
        Self::standard()
    }
}

impl Position {
    pub fn standard() -> Self {
        Self {
            board: STANDARD_BOARD,
            castle_rights: (
                CastleRights {
                    king: true,
                    queen: true,
                },
                CastleRights {
                    king: true,
                    queen: true,
                },
            ),
            to_play: Color::White,
            state: State::Playing,
            checks: vec![],
            king_coord: (Coord { row: 0, col: 4 }, Coord { row: 7, col: 4 }),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn to_play(&self) -> Color {
        self.to_play
    }

    pub fn castle_rights(&self, player: Color) -> CastleRights {
        match player {
            Color::White => self.castle_rights.0,
            Color::Black => self.castle_rights.1,
        }
    }

    fn castle_rights_mut(&mut self, player: Color) -> &mut CastleRights {
        match player {
            Color::White => &mut self.castle_rights.0,
            Color::Black => &mut self.castle_rights.1,
        }
    }

    pub fn king_coord(&self, player: Color) -> Coord {
        match player {
            Color::White => self.king_coord.0,
            Color::Black => self.king_coord.1,
        }
    }

    fn king_coord_mut(&mut self, player: Color) -> &mut Coord {
        match player {
            Color::White => &mut self.king_coord.0,
            Color::Black => &mut self.king_coord.1,
        }
    }

    pub fn is_in_check(&self) -> bool {
        !self.checks.is_empty()
    }

    pub fn get_attackers(&self, coord: Coord, player: Color) -> Vec<Coord> {
        let mut attackers = Vec::new();
        let opponent = match player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        for row in 0..8 {
            for col in 0..8 {
                let piece_coord = Coord { row, col };

                // Check if the piece belongs to the opponent
                if let Some(Square::Piece(piece, color)) = self.board.square(piece_coord) {
                    if color == opponent {
                        // Check if this piece can attack the given `coord`
                        if can_piece_attack(self.board, piece_coord, piece, color, coord) {
                            attackers.push(piece_coord);
                        }
                    }
                }
            }
        }

        todo!()
    }

    pub fn is_square_attacked(&self, coord: Coord, player: Color) -> bool {
        !self.get_attackers(coord, player).is_empty()
    }

    pub fn can_castle(&self, side: CastleSide) -> Option<MoveErr> {
        let player = self.to_play();

        let rights = self.castle_rights(player);

        let has_right = match side {
            CastleSide::King => rights.king,
            CastleSide::Queen => rights.queen,
        };

        if !has_right {
            return Some(MoveErr::NoCastlingRight);
        }

        if self.is_in_check() {
            return Some(MoveErr::KingInCheck);
        }

        let (row, king_col, rook_col) = match (player, side) {
            (Color::White, CastleSide::King) => (0, 4, 7),
            (Color::White, CastleSide::Queen) => (0, 4, 0),
            (Color::Black, CastleSide::King) => (7, 4, 7),
            (Color::Black, CastleSide::Queen) => (7, 4, 0),
        };

        let cols = if king_col < rook_col {
            king_col + 1..rook_col
        } else {
            rook_col + 1..king_col
        };

        for col in cols {
            let coord = Coord { row, col };
            let square = self.board.square(coord).unwrap();
            if !square.is_empty() {
                return Some(MoveErr::PathBlocked);
            }
        }

        for col in king_col..=rook_col {
            let coord = Coord { row, col };
            if self.is_square_attacked(coord, player) {
                return Some(MoveErr::KingInCheck);
            }
        }

        None
    }

    pub fn try_castle(&mut self, side: CastleSide) -> Result<(), MoveErr> {
        if let Some(err) = self.can_castle(side) {
            return Err(err);
        }

        let player = self.to_play();

        let (king_from, king_to, rook_from, rook_to) = match (player, side) {
            (Color::White, CastleSide::King) => (
                Coord { row: 0, col: 4 },
                Coord { row: 0, col: 6 },
                Coord { row: 0, col: 7 },
                Coord { row: 0, col: 5 },
            ),
            (Color::White, CastleSide::Queen) => (
                Coord { row: 0, col: 4 },
                Coord { row: 0, col: 2 },
                Coord { row: 0, col: 0 },
                Coord { row: 0, col: 3 },
            ),
            (Color::Black, CastleSide::King) => (
                Coord { row: 7, col: 4 },
                Coord { row: 7, col: 6 },
                Coord { row: 7, col: 7 },
                Coord { row: 7, col: 5 },
            ),
            (Color::Black, CastleSide::Queen) => (
                Coord { row: 7, col: 4 },
                Coord { row: 7, col: 2 },
                Coord { row: 7, col: 0 },
                Coord { row: 7, col: 3 },
            ),
        };

        self.board.move_piece(king_from, king_to);
        self.board.move_piece(rook_from, rook_to);

        *self.king_coord_mut(player) = king_to;
        *self.castle_rights_mut(player) = CastleRights {
            king: false,
            queen: false,
        };

        self.next_move();

        Ok(())
    }

    pub fn can_move(&self, from: Coord, to: Coord, promotion: Option<Piece>) -> Result<MoveInfo, MoveErr> {
        todo!()
    }

    pub fn try_move(
        &mut self,
        from: Coord,
        to: Coord,
        promotion: Option<Piece>,
    ) -> Result<MoveInfo, MoveErr> {
        let piece_move = self.can_move(from, to, promotion)?;

        todo!()
    }

    fn next_move(&mut self) {
        let next_player = match self.to_play {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.to_play = next_player;

        // TODO: Update state
        let king_coord = self.king_coord(next_player);

        self.checks = self.get_attackers(king_coord, next_player);
    }
}

fn can_piece_attack(board: Board, from: Coord, piece: Piece, color: Color, to: Coord) -> bool {
    match piece {
        Piece::Pawn => can_pawn_attack(from, to, color),
        Piece::Knight => can_knight_attack(from, to),
        Piece::Bishop => can_bishop_attack(board, from, to),
        Piece::Rook => can_rook_attack(board, from, to),
        Piece::Queen => can_queen_attack(board, from, to),
        Piece::King => can_king_attack(from, to),
    }
}

fn can_pawn_attack(from: Coord, to: Coord, color: Color) -> bool {
    let target_row = match color {
        Color::White => from.row + 1,
        Color::Black => from.row.wrapping_sub(1),
    };

    // A pawn attacks one row forward and one column to the left or right
    to.row == target_row && to.col.abs_diff(from.col) == 1
}

fn can_knight_attack(from: Coord, to: Coord) -> bool {
    // Knights move in "L" shapes
    from.row.abs_diff(to.row) * from.col.abs_diff(to.col) == 2
}

fn can_bishop_attack(board: Board, from: Coord, to: Coord) -> bool {
    // Bishops move diagonally
    let dy = from.row.abs_diff(to.row);
    let dx = from.col.abs_diff(to.col);

    dy == dx && is_clear_diagonal(board, from, to)
}

fn can_rook_attack(board: Board, from: Coord, to: Coord) -> bool {
    // Rooks move vertically or horizontally
    (from.row == to.row || from.col == to.col) && is_clear_line(board, from, to)
}

fn can_queen_attack(board: Board, from: Coord, to: Coord) -> bool {
    // Queens combine the movements of rooks and bishops
    can_bishop_attack(board, from, to) || can_rook_attack(board, from, to)
}

fn can_king_attack(from: Coord, to: Coord) -> bool {
    // Kings move one square in any direction
    let dy = from.row.abs_diff(to.row);
    let dx = from.col.abs_diff(to.col);

    dy <= 1 && dx <= 1
}

fn is_clear_line(board: Board, from: Coord, to: Coord) -> bool {
    if from.row == to.row {
        // Horizontal movement
        let (start, end) = if from.col < to.col {
            (from.col + 1, to.col)
        } else {
            (to.col + 1, from.col)
        };

        for col in start..end {
            let coord = Coord { row: from.row, col };
            if !board.square(coord).unwrap().is_empty() {
                return false;
            }
        }
    } else if from.col == to.col {
        // Vertical movement
        let (start, end) = if from.row < to.row {
            (from.row + 1, to.row)
        } else {
            (to.row + 1, from.row)
        };

        for row in start..end {
            let coord = Coord { row, col: from.col };
            if !board.square(coord).unwrap().is_empty() {
                return false;
            }
        }
    } else {
        // Not a line move
        return false;
    }

    true
}

fn is_clear_diagonal(board: Board, from: Coord, to: Coord) -> bool {
    // Check if the movement is diagonal
    let dy = from.row.abs_diff(to.row);
    let dx = from.col.abs_diff(to.col);
    if dy != dx {
        return false; // Not a diagonal move
    }

    // Determine the direction of movement
    let row_step = if to.row > from.row { 1 } else { -1 };
    let col_step = if to.col > from.col { 1 } else { -1 };

    let mut row = from.row;
    let mut col = from.col;

    while row != to.row && col != to.col {
        row = (row as i8 + row_step) as u8;
        col = (col as i8 + col_step) as u8;

        let coord = Coord { row, col };
        if let Some(square) = board.square(coord) {
            if !square.is_empty() {
                return false;
            }
        }
    }

    true
}
