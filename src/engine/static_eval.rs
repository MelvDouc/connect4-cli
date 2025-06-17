use std::collections::HashSet;

use crate::{
    bit_boards::{is_bit_set, pop_bit},
    constants::{NB_PLAYERS, cell::COL_MASKS, player::Player},
    position::{Direction as Dir, Position},
};

use super::score::MpScore;

fn visit(player_occ: u64, cell: u8, memo: &mut HashSet<u8>) -> bool {
    const DIRECTIONS: [Dir; 8] = [
        Dir::South,
        Dir::North,
        Dir::West,
        Dir::East,
        Dir::SouthWest,
        Dir::SouthEast,
        Dir::NorthWest,
        Dir::NorthEast,
    ];

    if memo.contains(&cell) || !is_bit_set(player_occ, cell) {
        return false;
    }

    memo.insert(cell);

    for dir in DIRECTIONS {
        if dir.distance_to_edge(cell) != 0 {
            visit(player_occ, dir.next_cell(cell), memo);
        }
    }

    true
}

fn count_islands(player_occ: u64) -> MpScore {
    let mut memo = HashSet::<u8>::new();
    let mut bb = player_occ;
    let mut count = 0;

    while bb != 0 {
        let cell = pop_bit(&mut bb);

        if visit(player_occ, cell, &mut memo) {
            count += 1;
        }
    }

    count
}

fn count_occupied_columns(player_occ: u64) -> MpScore {
    COL_MASKS.iter().fold(0, |acc, col_mask| {
        if player_occ & col_mask != 0 {
            return acc + 1;
        }

        acc
    })
}

fn eval_player(player_occ: u64) -> MpScore {
    const BONUS_COL_OCCUPIED: MpScore = 127;
    const MALUS_ISLAND: MpScore = 61;

    let mut score: MpScore = 0;
    score += count_occupied_columns(player_occ) * BONUS_COL_OCCUPIED;
    score -= count_islands(player_occ) * MALUS_ISLAND;
    score
}

pub(crate) fn static_eval(pos: &Position) -> MpScore {
    const PLAYER_MULTIPLIERS: [MpScore; NB_PLAYERS as usize] = [1, -1];

    let yellow_occ = pos.occupancy_of(Player::Yellow);
    let red_occ = pos.occupancy_of(Player::Red);

    let yellow_score = eval_player(yellow_occ);
    let red_score = eval_player(red_occ);

    (yellow_score - red_score) * PLAYER_MULTIPLIERS[pos.active_player() as usize]
}
