use crate::state::{Board, Board2};

mod constants;
mod move_gen;
mod state;
mod valid_board_gen;

fn main() {
    let board = Board2::new();

    println!("{}", board);
}
