use crate::bit_board_gen::print_bit_board_code;
use crate::bit_utils::get_bit_indices;
use crate::bit_utils::make_board;
use crate::bit_utils::print_bit_board;
use crate::constants::BoardIndex;
use crate::state::Board;
use crate::state::GameState;

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

    let moves = game_state.generate_moves();

    // [9->7, 9->11, 10->8, 10->12, 15->1, 15->29, 16->2, 16->14, 16->30, 17->3, 17->19, 18->4, 18->20]
    // [      9->11, 10->8,                        16->2, 16->14, 16->30, 17->3, 17->19]
    // [      9->11, 10->8,                                               17->3, 17->19]
    println!("{:?}", moves);
}
