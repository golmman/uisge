use crate::{state::Board, valid_board_gen::generate_valid_boards};

mod move_gen;
mod state;
mod valid_board_gen;

fn main() {
    let board = Board::new();

    println!("{}", board);

    generate_valid_boards();
}
