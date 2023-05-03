#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn enemy(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
}

