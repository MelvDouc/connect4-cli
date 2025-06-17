use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{
    constants::{NB_CELLS, NB_COLS},
    position::{GameResult, Position},
};

use super::{
    score::{LOSS_SCORE, MpScore},
    static_eval::static_eval,
    transposition::{Entry as TTEntry, Flag as TTFlag},
};

const NULL_MOVE: u8 = NB_CELLS;
const PSEUDO_INF: MpScore = MpScore::MAX - 1;

pub(crate) fn get_best_move(pos: &mut Position) -> u8 {
    let tt = &mut HashMap::<u64, TTEntry>::new();
    let depth = get_max_depth(pos);
    let ply = 0;
    let alpha = -PSEUDO_INF;
    let beta = PSEUDO_INF;
    let res = negamax(pos, tt, depth, ply, alpha, beta);

    match res.1 {
        Some(mv) => mv,
        None => panic!("Best move not found!"),
    }
}

fn negamax(
    pos: &mut Position,
    tt: &mut TranspositionTable,
    depth: usize,
    ply: u8,
    mut alpha: MpScore,
    mut beta: MpScore,
) -> NegamaxResult {
    let hash: u64 = pos.hash();

    if tt.contains_key(&hash) {
        let entry = tt.get(&hash).unwrap();

        if entry.depth >= depth {
            let score = entry.get_score(ply as MpScore);

            match entry.flag {
                TTFlag::Exact => {
                    return (score, None);
                }
                TTFlag::LowerBound => {
                    alpha = max(alpha, score);
                }
                TTFlag::UpperBound => {
                    beta = min(beta, score);
                }
            }

            if alpha >= beta {
                return (score, None);
            }
        }
    }

    negamax_result(pos, tt, depth, ply, alpha, beta)
}

fn negamax_result(
    pos: &mut Position,
    tt: &mut TranspositionTable,
    depth: usize,
    ply: u8,
    alpha: MpScore,
    beta: MpScore,
) -> NegamaxResult {
    match pos.result() {
        GameResult::Loss => {
            let score = LOSS_SCORE + ply as MpScore;
            let entry = TTEntry::new(TTFlag::Exact, score, depth);
            tt.insert(pos.hash(), entry);
            (score, None)
        }
        GameResult::Draw => {
            let score = 0;
            let entry = TTEntry::new(TTFlag::Exact, score, depth);
            tt.insert(pos.hash(), entry);
            (score, None)
        }
        GameResult::None => {
            if depth == 0 {
                return (static_eval(pos), None);
            }

            negamax_moves(pos, tt, depth, ply, alpha, beta)
        }
    }
}

fn negamax_moves(
    pos: &mut Position,
    tt: &mut TranspositionTable,
    depth: usize,
    ply: u8,
    mut alpha: MpScore,
    beta: MpScore,
) -> NegamaxResult {
    let start_alpha = alpha;
    let mut best_score = -PSEUDO_INF;
    let mut best_mv = NULL_MOVE;

    for mv in pos.legal_moves() {
        pos.play_move(mv);
        #[rustfmt::skip]
        let result = negamax(
            pos,
            tt,
            depth - 1,
            ply + 1,
            -beta,
            -alpha
        );
        let mv_score = -(result.0);
        pos.undo_move(mv);

        if mv_score <= best_score {
            continue;
        }

        best_score = mv_score;
        best_mv = mv;

        if best_score > alpha {
            alpha = best_score;

            if alpha >= beta {
                break;
            }
        }
    }

    let flag = TTEntry::get_flag(best_score, start_alpha, beta);
    let entry = TTEntry::new(flag, best_score, depth);
    tt.insert(pos.hash(), entry);
    (best_score, Some(best_mv))
}

fn get_max_depth(pos: &mut Position) -> usize {
    let ply_count = pos.full_occupancy().count_ones() as u8;

    if ply_count < NB_COLS {
        return 6;
    }

    10
}

type TranspositionTable = HashMap<u64, TTEntry>;
type NegamaxResult = (MpScore, Option<u8>);
