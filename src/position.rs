use crate::board::{Board, Color, Coord, Piece, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastleSide {
    King,
    Queen,
}

// -------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastleRights {
    pub king: bool,
    pub queen: bool,
}

// -------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Playing,
    Checkmate(Color),
    Stalemate(Color),
}

// -------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveErr {
    MoveToSameSquare,
    InvalidCoord,
    OriginSquareEmpty,
    DoesNotOwnOriginPiece,
    OwnsTargetPiece,
    InvalidMoveShape,
    PathNotEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastleErr {
    LackingPerms,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum MoveShape {
    OnlyMove,
    MoveAndAttack,
}

// -------------

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Move {
    pub origin: (Square, Coord),
    pub target: (Square, Coord),
    pub shape: MoveShape,
}

// -------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    board: Board,
    to_play: Color,
    castle_rights: (CastleRights, CastleRights),
    king_coord: (Coord, Coord),
    checks: Vec<(Square, Coord)>,
    state: State,
}

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

    pub fn state(&self) -> State {
        self.state
    }

    pub fn king_coord(&self) -> Coord {
        self.king_coord.0
    }

    pub fn checks(&self) -> &[(Square, Coord)] {
        self.checks.as_slice()
    }

    pub fn can_move_piece(
        &self,
        origin_coord: Coord,
        target_coord: Coord,
    ) -> Result<(Square, Square, MoveShape), MoveErr> {
        unimplemented!()
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

        todo!("checks, empty path")
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

        // TODO: Update king pos if king moved

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

        // Update castling permissions
        self.castle_rights.0 = CastleRights {
            king: false,
            queen: false,
        };

        Ok(self.next_turn())
    }

    fn next_turn(&mut self) -> State {
        self.to_play = match self.to_play {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.castle_rights = (self.castle_rights.1, self.castle_rights.0);
        self.state = State::Playing;

        // TODO: New state
        todo!("new state, checks")
    }
}

// -------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PositionErr {
    InvalidAmountOfKings,
    PawnsOnFirstRank,
    KingsTooClose,
    BothPlayersHaveChecks,
}

// -------------

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
    /// TODO: Refactor
    pub fn is_valid(&self) -> Result<(Coord, Coord), PositionErr> {
        // Kings
        let mut white_king = None;
        let mut black_king = None;

        let iter = self
            .board
            .iter(Coord::default())
            .filter(|(square, _)| square.piece_kind() == Some(Piece::King));

        for (square, coord) in iter {
            let king = match square.player().unwrap() {
                Color::White => &mut white_king,
                Color::Black => &mut black_king,
            };

            if king.replace((square, coord)).is_some() {
                return Err(PositionErr::InvalidAmountOfKings);
            }
        }

        let white_king = white_king.ok_or(PositionErr::InvalidAmountOfKings)?;
        let black_king = black_king.ok_or(PositionErr::InvalidAmountOfKings)?;

        // Kings too close
        let (dy, dx) = (
            white_king.1.row.abs_diff(black_king.1.row),
            white_king.1.col.abs_diff(black_king.1.col),
        );

        if dx <= 1 || dy <= 1 {
            return Err(PositionErr::KingsTooClose);
        }

        // Pawns on first rank
        let invalid_pawns = self
            .board
            .iter(Coord::default())
            .filter(|(_, coord)| matches!(coord.row, 0 | 7))
            .any(|(square, _)| square.piece_kind() == Some(Piece::Pawn));

        if invalid_pawns {
            return Err(PositionErr::PawnsOnFirstRank);
        }

        // Checks
        let checks_white: Vec<_> = iter_defenders(&self.board, white_king.1).collect();
        let checks_black: Vec<_> = iter_defenders(&self.board, black_king.1).collect();

        // TODO: Do something with checks
        if !checks_white.is_empty() && !checks_black.is_empty() {}

        todo!("checks")
    }

    pub fn try_build(self) -> Result<Position, PositionErr> {
        self.is_valid()?;

        unimplemented!()
    }
}

// -------------

// TODO: Is this code good?
fn move_shape(origin: (Square, Coord), target_coord: Coord) -> Option<MoveShape> {
    use {MoveShape::*, Piece::*};

    let (origin_square, origin_coord) = origin;
    let (origin_piece, origin_color) = origin_square.piece()?;

    let (dy, dx) = (
        (target_coord.row - origin_coord.row) as i8,
        (target_coord.col - origin_coord.col) as i8,
    );

    let (direction, row_2nd) = match origin_color {
        Color::White => (1, 1),
        Color::Black => (-1, 6),
    };

    let shape = match origin_piece {
        Pawn if dy == direction && dx == 0 => OnlyMove,
        Pawn if origin_coord.row == row_2nd && dy == direction * 2 && dx == 0 => OnlyMove,
        Pawn if dy == direction && dx.abs() == 1 => MoveAndAttack,
        Rook if dx * dy == 0 => MoveAndAttack,
        King if dx.abs() <= 1 && dy.abs() <= 1 => MoveAndAttack,
        Queen if dx * dy == 0 || dx.abs() == dy.abs() => MoveAndAttack,
        Knight if dx.abs() * dy.abs() == 2 => MoveAndAttack,
        Bishop if dx.abs() == dy.abs() => MoveAndAttack,
        _ => return None,
    };

    Some(shape)
}

fn can_move(
    board: &Board,
    player: Color,
    origin_coord: Coord,
    target_coord: Coord,
) -> Result<(Square, Square, MoveShape), MoveErr> {
    use MoveErr::*;

    // Origin coord cannot be target coord
    let origin_square = board.square(origin_coord).ok_or(InvalidCoord)?;
    let target_square = board.square(target_coord).ok_or(InvalidCoord)?;

    // Target coordinate cannot be origin coordinate
    if origin_coord != target_coord {
        return Err(MoveToSameSquare);
    }

    // Origin square must be non empty
    let (_, origin_player) = origin_square.piece().ok_or(OriginSquareEmpty)?;

    // Origin piece has to be owned by player
    if origin_player != player {
        return Err(DoesNotOwnOriginPiece);
    }

    // Target square cannot belong to player
    if target_square.player() == Some(player) {
        return Err(OwnsTargetPiece);
    }

    let move_shape = move_shape((origin_square, origin_coord), target_coord);

    match move_shape {
        None => return Err(InvalidMoveShape),
        Some(MoveShape::OnlyMove) if target_square != Square::Empty => {
            return Err(InvalidMoveShape)
        }
        _ => {}
    }

    // Path must be empty
    //if !is_path_between_clear(board, origin_coord, target_coord) {
    //    return Err(PathNotEmpty);
    //}

    todo!("checks")
}
// -------------

// TODO: Optimise
fn iter_defenders(
    board: &Board,
    origin_coord: Coord,
) -> impl Iterator<Item = (Square, Coord)> + '_ {
    board
        .iter(Coord::default())
        .filter(move |(square, coord)| match *square {
            Square::Empty => false,
            Square::Piece(Piece::King, _) => {
                defends(origin_coord, king_moves(board, (*square, *coord)))
            }
            Square::Piece(Piece::Pawn, _) => {
                defends(origin_coord, pawn_moves(board, (*square, *coord)))
            }
            Square::Piece(Piece::Rook, _) => {
                defends(origin_coord, rook_moves(board, (*square, *coord)))
            }
            Square::Piece(Piece::Queen, _) => {
                defends(origin_coord, queen_moves(board, (*square, *coord)))
            }
            Square::Piece(Piece::Bishop, _) => {
                defends(origin_coord, bishop_moves(board, (*square, *coord)))
            }
            Square::Piece(Piece::Knight, _) => {
                defends(origin_coord, knight_moves(board, (*square, *coord)))
            }
        })
}

// TODO: Ugly
fn defends<'a>(target_coord: Coord, iter: Option<impl Iterator<Item = Move> + 'a>) -> bool {
    if let Some(iter) = iter {
        iter.filter(|mov| mov.shape == MoveShape::MoveAndAttack)
            .any(move |mov| mov.target.1 == target_coord)
    } else {
        false
    }
}

// -------------

fn pawn_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (square, coord) = origin;
    let player = square.player()?;

    let (direction, row_2nd) = match player {
        Color::White => (1, 1),
        Color::Black => (-1, 6),
    };

    // move upwards
    let move_1_up = iter_square(board, increment_coord(coord, (direction, 0)));
    let move_2_up = iter_square(board, increment_coord(coord, (direction * 2, 0)))
        .filter(move |(_, coord)| coord.row == row_2nd);

    let moves = move_1_up.chain(move_2_up).map(move |target| Move {
        origin,
        target,
        shape: MoveShape::OnlyMove,
    });

    // attack diagonally
    let move_atk_left = iter_square(board, increment_coord(coord, (direction, 1)));
    let move_atk_right = iter_square(board, increment_coord(coord, (direction, -1)));

    let attacks = move_atk_left
        .chain(move_atk_right)
        .filter(move |(square, _)| square.player().filter(|p| *p != player).is_some())
        .map(move |target| Move {
            origin,
            target,
            shape: MoveShape::MoveAndAttack,
        });

    Some(moves.chain(attacks))
}

fn king_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (_, coord) = origin;

    let paths = iter_square(board, increment_coord(coord, (1, 1)))
        .chain(iter_square(board, increment_coord(coord, (1, 0))))
        .chain(iter_square(board, increment_coord(coord, (1, -1))))
        .chain(iter_square(board, increment_coord(coord, (0, 1))))
        .chain(iter_square(board, increment_coord(coord, (0, -1))))
        .chain(iter_square(board, increment_coord(coord, (-1, 1))))
        .chain(iter_square(board, increment_coord(coord, (-1, 0))))
        .chain(iter_square(board, increment_coord(coord, (-1, -1))));

    Some(paths.map(move |target| Move {
        origin,
        target,
        shape: MoveShape::MoveAndAttack,
    }))
}

fn knight_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (_, coord) = origin;

    let paths = iter_square(board, increment_coord(coord, (2, 1)))
        .chain(iter_square(board, increment_coord(coord, (2, -1))))
        .chain(iter_square(board, increment_coord(coord, (1, 2))))
        .chain(iter_square(board, increment_coord(coord, (1, -2))))
        .chain(iter_square(board, increment_coord(coord, (-1, 2))))
        .chain(iter_square(board, increment_coord(coord, (-1, -2))))
        .chain(iter_square(board, increment_coord(coord, (-2, 1))))
        .chain(iter_square(board, increment_coord(coord, (-2, -1))));

    Some(paths.map(move |target| Move {
        origin,
        target,
        shape: MoveShape::MoveAndAttack,
    }))
}

fn queen_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (square, coord) = origin;
    let player = square.player()?;

    let paths = iter_attack_path(board, player, coord, (1, 1))
        .chain(iter_attack_path(board, player, coord, (1, -1)))
        .chain(iter_attack_path(board, player, coord, (-1, 1)))
        .chain(iter_attack_path(board, player, coord, (-1, -1)))
        .chain(iter_attack_path(board, player, coord, (1, 0)))
        .chain(iter_attack_path(board, player, coord, (0, 1)))
        .chain(iter_attack_path(board, player, coord, (0, -1)))
        .chain(iter_attack_path(board, player, coord, (-1, -1)));

    Some(paths.map(move |target| Move {
        origin,
        target,
        shape: MoveShape::MoveAndAttack,
    }))
}

fn bishop_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (square, coord) = origin;
    let player = square.player()?;

    let paths = iter_attack_path(board, player, coord, (1, 1))
        .chain(iter_attack_path(board, player, coord, (1, -1)))
        .chain(iter_attack_path(board, player, coord, (-1, 1)))
        .chain(iter_attack_path(board, player, coord, (-1, -1)));

    Some(paths.map(move |target| Move {
        origin,
        target,
        shape: MoveShape::MoveAndAttack,
    }))
}

fn rook_moves(board: &Board, origin: (Square, Coord)) -> Option<impl Iterator<Item = Move> + '_> {
    let (square, coord) = origin;
    let player = square.player()?;

    let paths = iter_attack_path(board, player, coord, (1, 0))
        .chain(iter_attack_path(board, player, coord, (-1, 0)))
        .chain(iter_attack_path(board, player, coord, (0, 1)))
        .chain(iter_attack_path(board, player, coord, (0, -1)));

    Some(paths.map(move |target| Move {
        origin,
        target,
        shape: MoveShape::MoveAndAttack,
    }))
}

// -------------

fn iter_attack_path(
    board: &Board,
    player: Color,
    coord: Coord,
    increment: (i8, i8),
) -> impl Iterator<Item = (Square, Coord)> + '_ {
    let mut enemy_found = false;

    board
        .iter_path(coord, increment)
        .skip(1)
        .take_while(move |(square, _)| match *square {
            Square::Empty => true,
            Square::Piece(_, color) if color == player => {
                enemy_found = !enemy_found;
                enemy_found
            }
            _ => false,
        })
}

fn iter_square(board: &Board, coord: Coord) -> impl Iterator<Item = (Square, Coord)> {
    board.square(coord).zip(Some(coord)).into_iter()
}

fn increment_coord(coord: Coord, increment: (i8, i8)) -> Coord {
    Coord {
        row: coord.row.wrapping_add_signed(increment.0),
        col: coord.col.wrapping_add_signed(increment.1),
    }
}
