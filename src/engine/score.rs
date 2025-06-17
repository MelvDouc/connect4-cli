use crate::constants::NB_CELLS;

pub(super) const WIN_SCORE: MpScore = 1_000_000;
pub(super) const LOSS_SCORE: MpScore = -WIN_SCORE;

pub(super) const MAX_DEPTH: usize = NB_CELLS as usize;

pub(super) type MpScore = i32;
