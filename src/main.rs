use crate::gui::print_move_list_indices;
use crate::state::GameState;

use self::gui::start_gui;
use self::move_gen::Move;

mod bit_board_gen;
mod bit_utils;
mod constants;
mod gui;
mod move_gen;
mod piece_list;
mod state;
mod valid_board_gen;

fn main() {
    let mut game_state = GameState::new();

    start_gui(&mut game_state);
}
