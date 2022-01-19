use std::time::Instant;

use crate::constants::BoardIndex;
use crate::state::{Board, GameState};

mod constants;
mod move_gen;
mod piece_list;
mod state;
mod valid_board_gen;

fn get_bit_indices(i: u64) -> Vec<u8> {
    let mut indices = Vec::new();

    let mut j = i;

    loop {
        let index = j.trailing_zeros();
        if index == 64 {
            break;
        }

        indices.push(index as u8);

        j ^= (1 << index);
    }

    indices
}

fn main() {
    let game_state = GameState::new();

    println!("{}", game_state.board);

    println!("{}", 0u64.trailing_zeros());

    println!("{:?}", get_bit_indices(0b000001111011000000001));
}
