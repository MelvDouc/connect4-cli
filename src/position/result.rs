use crate::{
    bit_boards::pop_bit,
    constants::{NB_CELLS, NB_COLS, NB_CONNECTED, cell::mask_cell},
};

use super::direction::Direction as Dir;

#[derive(Clone, Copy)]
pub(crate) enum GameResult {
    Loss,
    Draw,
    None,
}

impl GameResult {
    #[rustfmt::skip]
    const DIRECTIONS: [Dir; 4] = [
        Dir::East,
        Dir::North,
        Dir::SouthEast,
        Dir::NorthEast
    ];

    const WIN_MASKS: [u64; 4] = {
        let mut arr = [0u64; 4];
        let mut i = 0;

        while i < NB_CONNECTED {
            arr[0] |= mask_cell(0, i);
            arr[1] |= mask_cell(i, 0);
            arr[2] |= mask_cell(i, NB_CONNECTED - i - 1);
            arr[3] |= mask_cell(i, i);
            i += 1;
        }

        arr
    };

    fn is_connected(player_occ: u64, dir_index: usize, cell: u8) -> bool {
        const MIN_DIST: u8 = NB_CONNECTED - 1;

        let dir = &Self::DIRECTIONS[dir_index];

        if dir.distance_to_edge(cell) < MIN_DIST {
            return false;
        }

        let win_mask = Self::WIN_MASKS[dir_index];
        let win_mask = match dir {
            Dir::SouthEast => win_mask << (cell - NB_COLS * (NB_CONNECTED - 1)), // diagonal
            _ => win_mask << cell,
        };

        player_occ & win_mask == win_mask
    }

    pub(crate) fn is_win(player_occ: u64) -> bool {
        let mut bb = player_occ;

        while bb != 0 {
            let cell = pop_bit(&mut bb);

            for dir_index in 0..Self::DIRECTIONS.len() {
                if Self::is_connected(player_occ, dir_index, cell) {
                    return true;
                }
            }
        }

        false
    }

    #[inline(always)]
    pub(crate) fn is_draw(full_occ: u64) -> bool {
        const FULL_BOARD_BB: u64 = (1 << NB_CELLS) - 1;

        full_occ == FULL_BOARD_BB
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bit_boards::bit_mask,
        constants::{NB_COLS, NB_CONNECTED, cell::cell_of},
    };

    use super::GameResult;

    #[test]
    fn should_find_row_win() {
        let cell = cell_of(1, 1);
        let mut bb = bit_mask(cell);

        for i in 1..NB_CONNECTED {
            assert!(!GameResult::is_win(bb));
            bb |= bit_mask(cell + i);
        }

        assert!(GameResult::is_win(bb));
    }

    #[test]
    fn should_find_diagonal_win() {
        let cell = cell_of(NB_CONNECTED - 1, 0);
        let mut bb = bit_mask(cell);

        for i in 1..NB_CONNECTED {
            assert!(!GameResult::is_win(bb));
            bb |= bit_mask(cell - (NB_COLS - 1) * i);
        }

        assert!(GameResult::is_win(bb));
    }

    #[test]
    fn should_find_antidiagonal_win() {
        let cell = cell_of(1, 1);
        let mut bb = bit_mask(cell);

        for i in 1..NB_CONNECTED {
            assert!(!GameResult::is_win(bb));
            bb |= bit_mask(cell + (NB_COLS + 1) * i);
        }

        assert!(GameResult::is_win(bb));
    }
}
