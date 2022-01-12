use crate::valid_board_gen::{
    benchmark_valid_board_hashing, is_connected, print_bit_board, read_valid_boards,
};
use crate::{state::Board, valid_board_gen::generate_valid_boards};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use self::constants::BitBoard;
use self::valid_board_gen::bytes_to_boards;

mod constants;
mod move_gen;
mod state;
mod valid_board_gen;

fn main() {
    let board = Board::new();

    println!("{}", board);
}
