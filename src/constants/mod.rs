pub(crate) mod cell;
pub(crate) mod player;

pub(crate) const NB_ROWS: u8 = 6;
pub(crate) const NB_COLS: u8 = 7;
pub(crate) const NB_CELLS: u8 = NB_ROWS * NB_COLS;

pub(crate) const NB_PLAYERS: u8 = 2;

/// The number of pieces that must be connected to win the game.
pub(crate) const NB_CONNECTED: u8 = 4;
