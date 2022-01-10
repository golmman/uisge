use crate::valid_board_gen::check_connected;
use crate::{state::Board, valid_board_gen::generate_valid_boards};

mod constants;
mod move_gen;
mod state;
mod valid_board_gen;

fn main() {
    let board = Board::new();

    println!("{}", board);

    //generate_valid_boards();
    check_connected(0b000000000000000000000000000001011111111111, 0);
}
