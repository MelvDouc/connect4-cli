use dialoguer::Select;

use crate::{
    bit_boards::is_bit_set,
    constants::{
        NB_COLS, NB_ROWS,
        cell::{cell_of, col_name_of, col_of, row_name_of, row_of},
        player::Player,
    },
    engine::get_best_move,
    position::{GameResult, Position},
};

const FG_RED: &str = "\x1b[31m";
const FG_YELLOW: &str = "\x1b[33m";
const CLR_RESET: &str = "\x1b[0m";

pub(crate) fn play_game() {
    const HUMAN: Player = Player::Yellow;
    const COMPUTER: Player = Player::Red;

    let pos = &mut Position::new();

    loop {
        let active_player = pos.active_player();
        print_pos(pos);
        println!();

        match pos.result() {
            GameResult::Loss => {
                println!("{} wins!", active_player.rev().symbol());
                break;
            }
            GameResult::Draw => {
                println!("The game is a draw.");
                break;
            }
            GameResult::None => {
                match active_player {
                    HUMAN => {
                        let mv = prompt_move(&pos.legal_moves());
                        pos.play_move(mv);
                    }
                    COMPUTER => {
                        play_computer_move(pos);
                    }
                };

                println!();
            }
        }
    }
}

fn play_computer_move(pos: &mut Position) {
    println!("{}Thinking...{}", FG_RED, CLR_RESET);
    let mv = get_best_move(pos);
    pos.play_move(mv);
    println!(
        "The computer played to: {}{}{}{}",
        FG_RED,
        col_name_of(col_of(mv)),
        row_name_of(row_of(mv)),
        CLR_RESET
    );
}

fn prompt_move(legal_moves: &Vec<u8>) -> u8 {
    let prompt = "What column will you play to?";
    let prompt = format!("{}{}{}", FG_YELLOW, prompt, CLR_RESET);
    let mut col_names: Vec<char> = vec![];

    for mv in legal_moves {
        let col_name = col_name_of(col_of(*mv));
        col_names.push(col_name);
    }

    let mv_index = Select::new()
        .with_prompt(prompt)
        .items(&col_names)
        .interact()
        .unwrap();
    legal_moves[mv_index]
}

fn print_pos(pos: &Position) {
    let yellow_occ = pos.occupancy_of(Player::Yellow);
    let red_occ = pos.occupancy_of(Player::Red);

    for row in (0..NB_ROWS).rev() {
        print!("{} ", row_name_of(row));

        for col in 0..NB_COLS {
            let cell = cell_of(row, col);

            if is_bit_set(yellow_occ, cell) {
                print!("{}", Player::Yellow.symbol());
                continue;
            }

            if is_bit_set(red_occ, cell) {
                print!("{}", Player::Red.symbol());
                continue;
            }

            print!("⚪");
        }

        println!();
    }

    print!("   ");

    for col in 0..NB_COLS {
        print!("{} ", col_name_of(col));
    }

    println!();
}
