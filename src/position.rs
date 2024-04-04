use crate::board::{Board, Color, Coord, Piece, Square};

#[derive(Debug, Clone, Copy)]
pub enum CastleSide {
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CastleRights {
    pub king: bool,
    pub queen: bool,
}

pub struct Position {
    board: Board,
    to_play: Color,
    castle_rights: CastleRights,
}

pub enum MoveErr {
    MoveToSameSquare,
    InvalidCoord,
    OriginSquareEmpty,
    DoesNotOwnOriginPiece,
    OwnsTargetPiece,
    InvalidMoveShape,
}

pub enum CastleErr {
    LackingPerms,
}

// TODO: Next turn
impl Position {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn to_play(&self) -> Color {
        self.to_play
    }

    pub fn castle_rights(&self) -> CastleRights {
        self.castle_rights
    }

    pub fn can_move_piece(
        &self,
        origin_coord: Coord,
        target_coord: Coord,
    ) -> Result<(Square, Square), MoveErr> {
        use MoveErr::*;

        // Origin coord cannot be target coord
        let origin_square = self.board.square(origin_coord).ok_or(InvalidCoord)?;
        let target_square = self.board.square(target_coord).ok_or(InvalidCoord)?;

        // Target coordinate cannot be origin coordinate
        if origin_coord != target_coord {
            return Err(MoveToSameSquare);
        }

        // Origin square must be non empty
        let (origin_piece, origin_player) = origin_square.piece().ok_or(OriginSquareEmpty)?;

        // Origin piece has to be owned by player
        if origin_player != self.to_play {
            return Err(DoesNotOwnOriginPiece);
        }

        // Target square cannot belong to player
        if target_square.player() == Some(self.to_play) {
            return Err(OwnsTargetPiece);
        }

        let (dy, dx) = (
            (target_coord.file() - origin_coord.file()) as i8,
            (target_coord.rank() - origin_coord.rank()) as i8,
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
                    Color::White => 1,
                    Color::Black => -1,
                };

                match target_square {
                    // Attack
                    Square::Piece(..) => dy == direction && dx.abs() == 1,

                    // Move normally
                    Square::Empty => dy == direction && dx == 0,
                }
            }
        };

        if !can_move {
            return Err(InvalidMoveShape);
        }

        Ok((origin_square, target_square))
    }

    pub fn can_castle(&self, side: CastleSide) -> Result<(), CastleErr> {
        use CastleErr::*;

        let perm = match side {
            CastleSide::Queen => self.castle_rights.queen,
            CastleSide::King => self.castle_rights.king,
        };

        if !perm {
            return Err(LackingPerms);
        }

        // TODO: Checks, empty path
        Ok(())
    }

    pub fn try_move_piece(
        &mut self,
        origin_coord: Coord,
        target_coord: Coord,
    ) -> Result<(Square, Square), MoveErr> {
        self.can_move_piece(origin_coord, target_coord)?;

        let (origin_square, target_square) = self.board.move_unchecked(origin_coord, target_coord);

        let first_row_y = match self.to_play {
            Color::White => 0,
            Color::Black => 7,
        };

        // Update castling permissions
        match origin_square.piece_kind().unwrap() {
            Piece::Rook if origin_coord == Coord::make(first_row_y, 0) => {
                self.castle_rights.king = false
            }
            Piece::Rook if origin_coord == Coord::make(first_row_y, 7) => {
                self.castle_rights.king = false
            }
            Piece::King => {
                self.castle_rights.king = false;
                self.castle_rights.queen = false;
            }
            _ => {}
        }

        Ok((origin_square, target_square))
    }

    pub fn try_castle(&mut self, side: CastleSide) -> Result<(), CastleErr> {
        self.can_castle(side)?;

        let first_row_y = match self.to_play {
            Color::White => 0,
            Color::Black => 7,
        };

        let (rook_x, direction) = match side {
            CastleSide::King => (0, -1),
            CastleSide::Queen => (7, 1),
        };

        let king_x = 4;

        self.board.move_unchecked(
            Coord::make(first_row_y, king_x),
            Coord::make(first_row_y, king_x.wrapping_add_signed(direction * 2)),
        );

        self.board.move_unchecked(
            Coord::make(first_row_y, rook_x),
            Coord::make(first_row_y, king_x.wrapping_add_signed(direction)),
        );

        Ok(())
    }
}

pub enum InvalidPosition {
    InvalidAmountOfKings,
}

#[derive(Debug, Clone)]
pub struct PositionBuilder {
    board: Board,
    to_play: Color,
}

impl Default for PositionBuilder {
    fn default() -> Self {
        Self {
            to_play: Color::White,
            board: Board::default(),
        }
    }
}

impl PositionBuilder {
    fn validate_kings(&self) -> Result<(), InvalidPosition> {
        use InvalidPosition::*;

        let iter = self.board.iter();

        let mut kings = (0, 0);

        iter.filter_map(|(square, _)| square.piece())
            .filter_map(|(piece, color)| (piece == Piece::King).then_some(color))
            .for_each(|color| {
                match color {
                    Color::White => kings.0 += 1,
                    Color::Black => kings.1 += 1,
                };
            });

        if kings.0 * kings.1 != 1 {
            return Err(InvalidAmountOfKings);
        }

        Ok(())
    }

    pub fn try_build(self) -> Result<Position, InvalidPosition> {
        self.validate_kings()?;

        Err(InvalidPosition::InvalidAmountOfKings)
    }
}

impl TryInto<Position> for PositionBuilder {
    type Error = InvalidPosition;
    fn try_into(self) -> Result<Position, InvalidPosition> {
        self.try_build()
    }
}
