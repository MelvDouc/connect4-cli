use super::score::{LOSS_SCORE, MAX_DEPTH, MpScore, WIN_SCORE};

pub(super) enum Flag {
    Exact,
    LowerBound,
    UpperBound,
}

pub(super) struct Entry {
    pub(super) flag: Flag,
    pub(super) depth: usize,
    score: MpScore,
}

impl Entry {
    pub(super) fn new(flag: Flag, score: MpScore, depth: usize) -> Self {
        Self { flag, score, depth }
    }

    pub(super) fn get_flag(score: MpScore, start_alpha: MpScore, beta: MpScore) -> Flag {
        if score <= start_alpha {
            return Flag::UpperBound;
        }

        if score >= beta {
            return Flag::LowerBound;
        }

        Flag::Exact
    }

    pub(super) fn get_score(&self, ply: MpScore) -> MpScore {
        const MD: MpScore = MAX_DEPTH as MpScore;

        if self.score + MD >= WIN_SCORE {
            return WIN_SCORE - ply;
        }

        if self.score - MD <= LOSS_SCORE {
            return LOSS_SCORE + ply;
        }

        self.score
    }
}
