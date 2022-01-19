use std::time::Instant;

use crate::bit_board_gen::print_bit_board_code;
use crate::bit_utils::{get_bit_indices, print_bit_board, make_board};
use crate::constants::BoardIndex;
use crate::state::{Board, GameState};

mod bit_board_gen;
mod bit_utils;
mod constants;
mod move_gen;
mod piece_list;
mod state;
mod valid_board_gen;

fn main() {
    let game_state = GameState::new();

    println!("{}", game_state.board);

    println!("{}", 0u64.trailing_zeros());

    println!("{:?}", get_bit_indices(0b000001111011000000001));

    print_bit_board_code();

    let x = make_board([
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1, 0],
    ]);
    print_bit_board(x);
}
