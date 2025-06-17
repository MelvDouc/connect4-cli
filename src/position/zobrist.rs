use crate::{
    bit_boards::Rng,
    constants::{NB_CELLS, NB_PLAYERS},
};

const NB_HASHES: usize = (NB_CELLS * NB_PLAYERS) as usize;

const HASHES: [u64; NB_HASHES] = {
    let mut arr = [0u64; NB_HASHES];
    let mut i = 0;
    let mut rng = Rng::new(Rng::SEEDS[1]);

    while i < NB_HASHES {
        arr[i] = rng.sparse_u64();
        i += 1;
    }

    arr
};

pub(crate) fn zobrist_hash(cell: u8, player: usize) -> u64 {
    let index = cell as usize + NB_CELLS as usize * player;
    HASHES[index]
}
