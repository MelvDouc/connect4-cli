use crate::{
    bit_boards::is_bit_set,
    constants::{NB_COLS, NB_ROWS, cell::cell_of},
};

pub(crate) fn get_legal_moves(full_occ: u64) -> Vec<u8> {
    let mut moves: Vec<u8> = vec![];

    for col in 0..NB_COLS {
        for row in 0..NB_ROWS {
            let cell = cell_of(row, col);

            if !is_bit_set(full_occ, cell) {
                moves.push(cell);
                break;
            }
        }
    }

    moves
}
