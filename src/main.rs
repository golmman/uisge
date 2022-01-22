use self::gui::start_gui;
use crate::state::GameState;

#[allow(unused)]
mod bit_board_gen;
#[allow(unused)]
mod bit_utils;
mod constants;
mod gui;
mod move_gen;
mod piece_list;
mod search;
mod state;
mod valid_board_gen;

fn main() {
    let mut game_state = GameState::new();

    start_gui(&mut game_state);
    //think(&game_state, 20);
}
