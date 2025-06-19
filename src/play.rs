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

const CLR_RESET: &str = "\x1b[0m";

pub(crate) fn play_game() {
    let human = prompt_human_side();
    let computer = human.rev();

    let pos = &mut Position::new();
    let mut computer_mv: Option<u8> = None;

    loop {
        let active_player = pos.active_player();

        clear_screen();
        print_pos(pos);

        if active_player == human {
            if let Some(computer_mv) = computer_mv {
                print_computer_move(computer_mv, computer.color());
            }
        }

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
                if active_player == human {
                    let mv = prompt_move(&pos.legal_moves(), human.color());
                    pos.play_move(mv);
                } else {
                    println!("{}Thinking...{}", computer.color(), CLR_RESET);
                    let mv = play_computer_move(pos);
                    computer_mv = Some(mv);
                }

                println!();
            }
        }
    }
}

fn prompt_human_side() -> Player {
    let index = Select::new()
        .with_prompt("Choose your color")
        .item(Player::Yellow.symbol())
        .item(Player::Red.symbol())
        .interact()
        .unwrap();

    match index {
        0 => Player::Yellow,
        1 => Player::Red,
        _ => panic!("Invalid player color."),
    }
}

fn play_computer_move(pos: &mut Position) -> u8 {
    let mv = get_best_move(pos);
    pos.play_move(mv);
    mv
}

fn print_computer_move(mv: u8, color: &str) {
    println!(
        "The computer played to {}{}{}{}.",
        color,
        col_name_of(col_of(mv)),
        row_name_of(row_of(mv)),
        CLR_RESET
    );
}

fn prompt_move(legal_moves: &Vec<u8>, color: &str) -> u8 {
    let prompt = "Choose your move";
    let prompt = format!("{}{}{}", color, prompt, CLR_RESET);
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

            print!("{}", Player::EMPTY_SYMBOL);
        }

        println!();
    }

    print!("   ");

    for col in 0..NB_COLS {
        print!("{} ", col_name_of(col));
    }

    println!("\n");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
