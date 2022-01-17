use crate::state::{Board, GameState};

mod constants;
mod move_gen;
mod piece_list;
mod state;
mod valid_board_gen;

fn main() {
    let game_state = GameState::new();

    println!("{}", game_state.board);
}
