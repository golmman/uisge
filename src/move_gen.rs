use crate::constants::{BitBoard, BoardIndex};
use crate::state::GameState;

// note that 'move' is a rust keyword, so when intended as variable 'mov' is used instead here
pub struct Move {
    from: BoardIndex,
    to: BoardIndex,
}

pub fn generate_moves(game_state: &GameState) -> Vec<Move> {
    let moves = Vec::<Move>::new();
    let board = &game_state.board;

    //for

    moves
}

pub fn make_move(bit_board: BitBoard, mov: Move) -> BitBoard {
    1
}

pub fn unmake_move(bit_board: BitBoard, mov: Move) -> BitBoard {
    1
}
