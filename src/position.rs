use crate::board::{Board, Color, Coord, Piece, Square};
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastleSide {
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastleRights {
    pub king: bool,
    pub queen: bool,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Playing,
    Checkmate(Color),
    Stalemate(Color),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    ) -> Result<(Square, Square, MoveShape), MoveErr> {
        can_move(&self.board, self.to_play, origin_coord, target_coord)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PositionErr {
    InvalidAmountOfKings,
    PawnsOnFirstRank,
    KingsTooClose,
    BothPlayersHaveChecks,
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
            .filter(|(_, coord)| matches!(coord.rank, 0 | 7))
            .filter_map(|(square, _)| square.piece())
            .any(|(piece, _)| piece == Piece::Pawn);

        if pawns_exist {
            return Err(PositionErr::PawnsOnFirstRank);
        }

        // Kings too close
        let (dx, dy) = (
            kings.1.file.abs_diff(kings.0.file),
            kings.1.rank.abs_diff(kings.1.file),
        );

        if dx <= 1 || dy <= 1 {
            return Err(PositionErr::KingsTooClose);
        }

        // TODO: Passant
        // TODO: Checks
        let checks: Vec<_> = all_moves(&self.board)
            .filter(|(.., shape)| *shape == MoveShape::MoveAndAttack)
            .filter(|(_, (_, target_coord), _)| {
                *target_coord == kings.0 || *target_coord == kings.1
            })
            .map(|(origin, target, _)| (origin, target))
            .collect();

        let mut check_test = (false, false);

        for ((origin_square, _), ..) in checks.iter() {
            let player = origin_square.player().unwrap();

            let test = match player {
                Color::White => check_test.0 = true,
                Color::Black => check_test.1 = true,
            };

            if check_test.0 && check_test.1 {
                break;
            }
        }

        if check_test.0 && check_test.1 {
            return Err(PositionErr::BothPlayersHaveChecks);
        }

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum MoveShape {
    OnlyMove,
    MoveAndAttack,
    NoMove,
}

fn move_shape(piece: (Piece, Color), origin_coord: Coord, target_coord: Coord) -> MoveShape {
    use {MoveShape::*, Piece::*};

    let (dy, dx) = (
        (target_coord.file - origin_coord.file) as i8,
        (target_coord.rank - origin_coord.rank) as i8,
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
        Pawn if origin_coord.rank == rank_2nd && dy == direction * 2 && dx == 0 => OnlyMove,
        Pawn if dy == direction && dx.abs() == 1 => MoveAndAttack,
        Rook if dx * dy == 0 => MoveAndAttack,
        King if dx.abs() <= 1 && dy.abs() <= 1 => MoveAndAttack,
        Queen if dx * dy == 0 || dx.abs() == dy.abs() => MoveAndAttack,
        Knight if dx.abs() * dy.abs() == 2 => MoveAndAttack,
        Bishop if dx.abs() == dy.abs() => MoveAndAttack,
        _ => NoMove,
    }
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
    let (origin_piece, origin_player) = origin_square.piece().ok_or(OriginSquareEmpty)?;

    // Origin piece has to be owned by player
    if origin_player != player {
        return Err(DoesNotOwnOriginPiece);
    }

    // Target square cannot belong to player
    if target_square.player() == Some(player) {
        return Err(OwnsTargetPiece);
    }

    let move_shape = move_shape((origin_piece, origin_player), origin_coord, target_coord);

    match move_shape {
        MoveShape::NoMove => return Err(InvalidMoveShape),
        MoveShape::OnlyMove if target_square != Square::Empty => return Err(InvalidMoveShape),
        _ => {}
    }

    // Path must be empty
    if !is_path_clear(board, origin_coord, target_coord) {
        return Err(PathNotEmpty);
    }

    // TODO: Checks

    // TODO: En passant

    Ok((origin_square, target_square, move_shape))
}

enum MoveType {
    AttackAndMove,
    Move,
}

// TODO: Optimise to iterate over board once
#[inline(always)]
fn all_moves(
    board: &Board,
) -> impl Iterator<Item = ((Square, Coord), (Square, Coord), MoveShape)> + '_ {
    queen_moves(board)
        .chain(knight_moves(board))
        .chain(bishop_moves(board))
        .chain(king_moves(board))
        .chain(rook_moves(board))
        .map(|(origin, target)| (origin, target, MoveShape::MoveAndAttack))
        .chain(pawn_moves(board))
}

// TODO: Feed iterator of piece locations prior
#[inline(always)]
fn pawn_moves(
    board: &Board,
) -> impl Iterator<Item = ((Square, Coord), (Square, Coord), MoveShape)> + '_ {
    iter_piece(board, Piece::Pawn).flat_map(move |(origin_square, origin_coord)| {
        let player = origin_square.player().unwrap();

        let direction = match player {
            Color::White => 1,
            Color::Black => -1,
        };

        let rank_2nd = match player {
            Color::White => 1,
            Color::Black => 6,
        };

        // move upwards
        let coord_1_up = increment_coord(origin_coord, (direction, 0));
        let move_1_up = board.square(coord_1_up).zip(Some(coord_1_up));

        let coord_2_up = increment_coord(origin_coord, (direction * 2, 0));
        let move_2_up = (origin_coord.rank == rank_2nd)
            .then_some(board.square(coord_2_up).zip(Some(coord_2_up)))
            .flatten();

        let moves = move_1_up
            .into_iter()
            .chain(move_2_up.into_iter())
            .zip(iter::repeat(MoveShape::OnlyMove));

        // attack diagonally
        let coord_atk_left = increment_coord(origin_coord, (direction, -1));
        let move_atk_left = board.square(coord_atk_left).zip(Some(coord_atk_left));

        let coord_atk_right = increment_coord(origin_coord, (direction, 1));
        let move_atk_right = board.square(coord_atk_right).zip(Some(coord_atk_right));

        let attacks = move_atk_left
            .into_iter()
            .chain(move_atk_right.into_iter())
            .zip(iter::repeat(MoveShape::MoveAndAttack));

        moves
            .chain(attacks)
            .map(move |(target, shape)| ((origin_square, origin_coord), target, shape))
    })
}

#[inline(always)]
fn king_moves(board: &Board) -> impl Iterator<Item = ((Square, Coord), (Square, Coord))> + '_ {
    iter_piece(board, Piece::King).flat_map(move |(origin_square, origin_coord)| {
        let rank = -1..=1;
        let file = -1..=1;

        let combinations = file
            .flat_map(move |i| rank.clone().map(move |j| (i, j)))
            .filter(|(i, j)| *i != 0 && *j != 0);

        let paths = combinations.filter_map(move |increment| {
            let target_coord = increment_coord(origin_coord, increment);
            board.square(target_coord).zip(Some(target_coord))
        });

        iter::repeat((origin_square, origin_coord)).zip(paths)
    })
}

#[inline(always)]
fn knight_moves(board: &Board) -> impl Iterator<Item = ((Square, Coord), (Square, Coord))> + '_ {
    iter_piece(board, Piece::Knight).flat_map(move |(origin_square, origin_coord)| {
        let rank = (-2..=1).chain(1..=2);
        let file = (-2..=1).chain(1..=2);

        let combinations = file.flat_map(move |i| rank.clone().map(move |j| (i, j)));
        let paths = combinations.filter_map(move |increment| {
            let target_coord = increment_coord(origin_coord, increment);
            board.square(target_coord).zip(Some(target_coord))
        });

        iter::repeat((origin_square, origin_coord)).zip(paths)
    })
}

#[inline(always)]
fn queen_moves(board: &Board) -> impl Iterator<Item = ((Square, Coord), (Square, Coord))> + '_ {
    iter_piece(board, Piece::Queen).flat_map(|(origin_square, origin_coord)| {
        let player = origin_square.player().unwrap();

        let bishop_paths = iter_path(board, player, origin_coord, (1, 1))
            .chain(iter_path(board, player, origin_coord, (1, -1)))
            .chain(iter_path(board, player, origin_coord, (-1, 1)))
            .chain(iter_path(board, player, origin_coord, (-1, -1)));

        let rook_paths = iter_path(board, player, origin_coord, (1, 0))
            .chain(iter_path(board, player, origin_coord, (-1, 0)))
            .chain(iter_path(board, player, origin_coord, (0, 1)))
            .chain(iter_path(board, player, origin_coord, (0, -1)));

        let paths = bishop_paths.chain(rook_paths);

        iter::repeat((origin_square, origin_coord)).zip(paths)
    })
}

#[inline(always)]
fn bishop_moves(board: &Board) -> impl Iterator<Item = ((Square, Coord), (Square, Coord))> + '_ {
    iter_piece(board, Piece::Bishop).flat_map(|(origin_square, origin_coord)| {
        let player = origin_square.player().unwrap();

        let paths = iter_path(board, player, origin_coord, (1, 1))
            .chain(iter_path(board, player, origin_coord, (1, -1)))
            .chain(iter_path(board, player, origin_coord, (-1, 1)))
            .chain(iter_path(board, player, origin_coord, (-1, -1)));

        iter::repeat((origin_square, origin_coord)).zip(paths)
    })
}

#[inline(always)]
fn rook_moves(board: &Board) -> impl Iterator<Item = ((Square, Coord), (Square, Coord))> + '_ {
    iter_piece(board, Piece::Rook).flat_map(|(origin_square, origin_coord)| {
        let player = origin_square.player().unwrap();

        let paths = iter_path(board, player, origin_coord, (1, 0))
            .chain(iter_path(board, player, origin_coord, (-1, 0)))
            .chain(iter_path(board, player, origin_coord, (0, 1)))
            .chain(iter_path(board, player, origin_coord, (0, -1)));

        iter::repeat((origin_square, origin_coord)).zip(paths)
    })
}

#[inline(always)]
fn iter_path(
    board: &Board,
    player: Color,
    coord: Coord,
    increment: (i8, i8),
) -> impl Iterator<Item = (Square, Coord)> + '_ {
    let mut enemy_found = false;

    board
        .path_iter(coord, increment)
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

#[inline(always)]
fn iter_piece(board: &Board, target: Piece) -> impl Iterator<Item = (Square, Coord)> + '_ {
    board
        .iter()
        .filter(move |(square, _)| square.piece_kind() == Some(target))
}

#[inline(always)]
fn iter_pieces(board: &Board) -> impl Iterator<Item = (Square, Coord)> + '_ {
    board
        .iter()
        .filter(|(square, _)| square.piece_kind().is_some())
}

// TODO: Rewrite
fn is_path_clear(board: &Board, origin_coord: Coord, target_coord: Coord) -> bool {
    let increment = (
        (target_coord.file as i8 - origin_coord.file as i8).signum(),
        (target_coord.rank as i8 - origin_coord.rank as i8).signum(),
    );

    // Ensure all squares are empty
    board
        .path_iter(origin_coord, increment)
        .skip(1)
        .take_while(|(_, coord)| coord.file != target_coord.file && coord.rank != target_coord.rank)
        .all(|(square, _)| square == Square::Empty)
}

fn increment_coord(coord: Coord, increment: (i8, i8)) -> Coord {
    Coord {
        file: coord.file.wrapping_add_signed(increment.0),
        rank: coord.rank.wrapping_add_signed(increment.1),
    }
}
