use crate::constants::{
    NB_COLS,
    cell::{col_of, rev_col, rev_row, row_of},
};

/// Note: `std::cmd::min` is not constant.
const fn min(a: u8, b: u8) -> u8 {
    if a < b { a } else { b }
}

pub(crate) enum Direction {
    South,
    North,
    West,
    East,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast,
}

impl Direction {
    pub(crate) const fn _opposite(&self) -> Direction {
        match self {
            Self::South => Self::North,
            Self::North => Self::South,
            Self::West => Self::East,
            Self::East => Self::West,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
            Self::NorthWest => Self::SouthEast,
            Self::NorthEast => Self::SouthWest,
        }
    }

    pub(crate) const fn distance_to_edge(&self, cell: u8) -> u8 {
        self.dte(cell)
    }

    pub(crate) const fn next_cell(&self, cell: u8) -> u8 {
        match self {
            Self::South => cell - NB_COLS,
            Self::North => cell + NB_COLS,
            Self::West => cell - 1,
            Self::East => cell + 1,
            Self::SouthWest => cell - NB_COLS - 1,
            Self::SouthEast => cell - NB_COLS + 1,
            Self::NorthWest => cell + NB_COLS - 1,
            Self::NorthEast => cell + NB_COLS + 1,
        }
    }

    const fn dte(&self, cell: u8) -> u8 {
        match self {
            Self::South => row_of(cell),
            Self::North => rev_row(row_of(cell)),
            Self::West => col_of(cell),
            Self::East => rev_col(col_of(cell)),
            Self::SouthWest => min(Self::South.dte(cell), Self::West.dte(cell)),
            Self::SouthEast => min(Self::South.dte(cell), Self::East.dte(cell)),
            Self::NorthWest => min(Self::North.dte(cell), Self::West.dte(cell)),
            Self::NorthEast => min(Self::North.dte(cell), Self::East.dte(cell)),
        }
    }
}
