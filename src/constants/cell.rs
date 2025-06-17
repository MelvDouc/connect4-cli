use crate::{
    bit_boards::bit_mask,
    constants::{NB_COLS, NB_ROWS},
};

#[inline(always)]
pub(crate) const fn mask_cell(row: u8, col: u8) -> u64 {
    bit_mask(cell_of(row, col))
}

#[inline(always)]
pub(crate) const fn row_of(cell: u8) -> u8 {
    cell / NB_COLS
}

#[inline(always)]
pub(crate) const fn col_of(cell: u8) -> u8 {
    cell % NB_COLS
}

#[inline(always)]
pub(crate) const fn cell_of(row: u8, col: u8) -> u8 {
    row * NB_COLS + col
}

pub(crate) const fn rev_row(row: u8) -> u8 {
    NB_ROWS - row - 1
}

pub(crate) const fn rev_col(col: u8) -> u8 {
    NB_COLS - col - 1
}

pub(crate) const COL_MASKS: [u64; NB_COLS as usize] = {
    let mut arr = [0u64; NB_COLS as usize];
    let mut row: u8 = 0;
    let mut col: u8 = 0;

    while row < NB_ROWS {
        arr[col as usize] |= mask_cell(row, col);
        row += 1;
    }

    col += 1;

    while col < NB_COLS {
        let c = col as usize;
        arr[c] = arr[c - 1] << 1;
        col += 1;
    }

    arr
};

pub(crate) const fn row_name_of(row: u8) -> char {
    const ROW_NAMES: [char; NB_ROWS as usize] = ['1', '2', '3', '4', '5', '6'];

    ROW_NAMES[row as usize]
}

pub(crate) const fn col_name_of(col: u8) -> char {
    const COL_NAMES: [char; NB_COLS as usize] = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];

    COL_NAMES[col as usize]
}
