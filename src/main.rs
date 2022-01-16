use crate::state::Board;

mod constants;
mod move_gen;
mod state;
mod valid_board_gen;

fn main() {
    let board = Board::new();

    println!("{}", board);
}
