use crate::{
    bit_boards::{bit_clear, bit_mask},
    constants::{NB_PLAYERS, player::Player},
};

use super::{GameResult, moves::get_legal_moves, zobrist::zobrist_hash};

pub(crate) struct Position {
    board_: [u64; NB_PLAYERS as usize],
    active_player_: Player,
    hash_: u64,
}

impl Position {
    pub(crate) const fn new() -> Self {
        let board = [0u64; NB_PLAYERS as usize];
        Self {
            board_: board,
            active_player_: Player::Yellow,
            hash_: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn active_player(&self) -> Player {
        self.active_player_
    }

    #[inline(always)]
    pub(crate) const fn occupancy_of(&self, player: Player) -> u64 {
        self.board_[player as usize]
    }

    #[inline(always)]
    pub(crate) const fn full_occupancy(&self) -> u64 {
        self.occupancy_of(Player::Yellow) | self.occupancy_of(Player::Red)
    }

    #[inline(always)]
    pub(crate) const fn hash(&self) -> u64 {
        self.hash_
    }

    pub(crate) fn result(&self) -> GameResult {
        let inactive_occ = self.occupancy_of(self.active_player_.rev());

        if GameResult::is_win(inactive_occ) {
            return GameResult::Loss;
        }

        if GameResult::is_draw(self.full_occupancy()) {
            return GameResult::Draw;
        }

        GameResult::None
    }

    pub(crate) fn legal_moves(&self) -> Vec<u8> {
        get_legal_moves(self.full_occupancy())
    }

    pub(crate) fn play_move(&mut self, mv: u8) {
        let player = self.active_player_ as usize;
        self.board_[player] |= bit_mask(mv);
        self.hash_ ^= zobrist_hash(mv, player);
        self.active_player_ = self.active_player_.rev();
    }

    pub(crate) fn undo_move(&mut self, mv: u8) {
        let inactive_player = self.active_player_.rev();
        self.board_[inactive_player as usize] &= bit_clear(mv);
        self.hash_ ^= zobrist_hash(mv, inactive_player as usize);
        self.active_player_ = inactive_player;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::seq::IndexedRandom;

    use super::*;

    #[test]
    fn unique_hashes() {
        let mut pos = Position::new();
        let mut hashes = HashSet::<u64>::new();
        let mut rng = rand::rng();

        loop {
            let legal_moves = pos.legal_moves();
            let mv = legal_moves.choose(&mut rng);
            let mv = *(mv.unwrap());
            pos.play_move(mv);

            match pos.result() {
                GameResult::Loss | GameResult::Draw => {
                    break;
                }
                GameResult::None => {
                    let hash = pos.hash();
                    assert!(!hashes.contains(&hash));
                    hashes.insert(hash);
                }
            }
        }
    }
}
