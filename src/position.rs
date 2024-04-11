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

#[derive(Debug, Clone, Copy)]
pub enum State {
    Playing,
    Checkmate(Color),
    Stalemate(Color),
}

pub struct Position {
    board: Board,
    to_play: Color,
    castle_rights: (CastleRights, CastleRights),
    state: State,
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
        self.castle_rights.0
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

        let move_shape = move_shape((origin_piece, origin_player), origin_coord, target_coord);

        match move_shape {
            MoveShape::NoMove => return Err(InvalidMoveShape),
            MoveShape::OnlyMove if target_square != Square::Empty => return Err(InvalidMoveShape),
            _ => {}
        }

        Ok((origin_square, target_square))
    }

    pub fn can_castle(&self, side: CastleSide) -> Result<(), CastleErr> {
        use CastleErr::*;

        let perm = match side {
            CastleSide::Queen => self.castle_rights.0.queen,
            CastleSide::King => self.castle_rights.0.king,
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
    ) -> Result<(State, Square, Square), MoveErr> {
        self.can_move_piece(origin_coord, target_coord)?;

        let (origin_square, target_square) = self.board.move_unchecked(origin_coord, target_coord);

        let first_row_y = match self.to_play {
            Color::White => 0,
            Color::Black => 7,
        };

        // Update castling permissions
        match origin_square.piece_kind().unwrap() {
            Piece::Rook if origin_coord == Coord::make(first_row_y, 0) => {
                self.castle_rights.0.king = false
            }
            Piece::Rook if origin_coord == Coord::make(first_row_y, 7) => {
                self.castle_rights.0.king = false
            }
            Piece::King => {
                self.castle_rights.0.king = false;
                self.castle_rights.0.queen = false;
            }
            _ => {}
        }

        Ok((self.next_turn(), origin_square, target_square))
    }

    pub fn try_castle(&mut self, side: CastleSide) -> Result<State, CastleErr> {
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

        Ok(self.next_turn())
    }

    fn next_turn(&mut self) -> State {
        self.to_play = match self.to_play {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.castle_rights = (self.castle_rights.1, self.castle_rights.0);
        self.state = State::Playing;

        self.state

        // TODO: New state
    }
}

pub enum PositionErr {
    InvalidAmountOfKings,
    PawnsOnFirstRank,
    KingsTooClose,
}

#[derive(Debug, Clone)]
pub struct PositionBuilder {
    board: Board,
    to_play: Color,
}

// TODO: Default board setup
impl Default for PositionBuilder {
    fn default() -> Self {
        Self {
            to_play: Color::White,
            board: Board::default(),
        }
    }
}

impl PositionBuilder {
    pub fn is_valid(&self) -> Result<(Coord, Coord), PositionErr> {
        // Validate kings
        let iter = self
            .board
            .iter()
            .filter_map(|(square, coord)| {
                square.piece().map(|(piece, color)| (piece, color, coord))
            })
            .filter_map(|(piece, color, coord)| (piece == Piece::King).then_some((color, coord)));

        let mut kings = (None, None);

        for (color, coord) in iter {
            let king = match color {
                Color::White => &mut kings.0,
                Color::Black => &mut kings.1,
            };

            if king.replace(coord).is_some() {
                return Err(PositionErr::InvalidAmountOfKings);
            }
        }

        let kings = (
            kings.0.ok_or(PositionErr::InvalidAmountOfKings)?,
            kings.1.ok_or(PositionErr::InvalidAmountOfKings)?,
        );

        // Pawns on first rank
        let pawns_exist = self
            .board
            .iter()
            .filter(|(_, coord)| matches!(coord.rank(), 0 | 7))
            .filter_map(|(square, _)| square.piece())
            .any(|(piece, _)| piece == Piece::Pawn);

        if pawns_exist {
            return Err(PositionErr::PawnsOnFirstRank);
        }

        // Kings too close
        let (dx, dy) = (
            kings.1.file().abs_diff(kings.0.file()),
            kings.1.rank().abs_diff(kings.1.file()),
        );

        if dx <= 1 || dy <= 1 {
            return Err(PositionErr::KingsTooClose);
        }

        // TODO: Passant
        // TODO: Checks
        Ok(kings)
    }

    pub fn try_build(self) -> Result<Position, PositionErr> {
        self.is_valid()?;

        Err(PositionErr::InvalidAmountOfKings)
    }
}

impl TryInto<Position> for PositionBuilder {
    type Error = PositionErr;
    fn try_into(self) -> Result<Position, PositionErr> {
        self.try_build()
    }
}

enum MoveShape {
    OnlyMove,
    MoveAndAttack,
    NoMove,
}

fn move_shape(piece: (Piece, Color), origin_coord: Coord, target_coord: Coord) -> MoveShape {
    use {MoveShape::*, Piece::*};

    let (dy, dx) = (
        (target_coord.file() - origin_coord.file()) as i8,
        (target_coord.rank() - origin_coord.rank()) as i8,
    );

    let direction = match piece.1 {
        Color::White => 1,
        Color::Black => -1,
    };

    let rank_2nd = match piece.1 {
        Color::White => 1,
        Color::Black => 6,
    };

    match piece.0 {
        Pawn if dy == direction && dx == 0 => OnlyMove,
        Pawn if origin_coord.rank() == rank_2nd && dy == direction * 2 && dx == 0 => OnlyMove,
        Pawn if dy == direction && dx.abs() == 1 => MoveAndAttack,
        Rook if dx * dy == 0 => MoveAndAttack,
        King if dx.abs() <= 1 && dy.abs() <= 1 => MoveAndAttack,
        Queen if dx * dy == 0 || dx.abs() == dy.abs() => MoveAndAttack,
        Knight if dx.abs() * dy.abs() == 2 => MoveAndAttack,
        Bishop if dx.abs() == dy.abs() => MoveAndAttack,
        _ => NoMove,
    }
}
