#[inline(always)]
pub(crate) const fn bit_mask(cell: u8) -> u64 {
    1u64 << cell
}

#[inline(always)]
pub(crate) const fn bit_clear(cell: u8) -> u64 {
    !bit_mask(cell)
}

#[inline(always)]
pub(crate) const fn is_bit_set(bb: u64, cell: u8) -> bool {
    bb & bit_mask(cell) != 0
}

pub(crate) const fn pop_bit(bb: &mut u64) -> u8 {
    let cell = (*bb).trailing_zeros() as u8;
    *bb &= bit_clear(cell);
    cell
}
