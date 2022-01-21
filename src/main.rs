use std::io::stdin;

use crate::state::GameState;

use self::move_gen::Move;

mod bit_board_gen;
mod bit_utils;
mod constants;
mod move_gen;
mod piece_list;
mod state;
mod valid_board_gen;

fn get_move_list_indices(moves: Vec<Move>) -> String {
    let mut indices = String::new();

    for i in 0..moves.len() {
        indices.push_str(&format!("   {:02}   ", i));
    }

    indices
}

fn main() {
    let game_state = GameState::new();

    println!("{}", game_state.board);

    let moves = game_state.generate_moves();

    // [9->7, 9->11, 10->8, 10->12, 15->1, 15->29, 16->2, 16->14, 16->30, 17->3, 17->19, 18->4, 18->20]
    // [      9->11, 10->8,                        16->2, 16->14, 16->30, 17->3, 17->19]
    // [      9->11, 10->8,                                               17->3, 17->19]
    println!("{:?}", moves);
    println!("{}", get_move_list_indices(moves));

    let mut buffer = String::new();
    let mut stdin = stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();
}
