#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Player {
    Yellow,
    Red,
}

impl Player {
    const FG_RED: &str = "\x1b[31m";
    const FG_YELLOW: &str = "\x1b[33m";

    pub(crate) const EMPTY_SYMBOL: char = '⚪';

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

    pub(crate) fn color(&self) -> &str {
        match self {
            Player::Yellow => Self::FG_YELLOW,
            Player::Red => Self::FG_RED,
        }
    }
}
