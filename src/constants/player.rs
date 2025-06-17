#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Player {
    Yellow,
    Red,
}

impl Player {
    pub(crate) fn rev(&self) -> Self {
        match self {
            Self::Yellow => Self::Red,
            Self::Red => Self::Yellow,
        }
    }

    pub(crate) fn symbol(&self) -> char {
        match self {
            Player::Yellow => '🟡',
            Player::Red => '🔴',
        }
    }
}
