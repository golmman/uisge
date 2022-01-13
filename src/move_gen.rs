use crate::constants::BitBoard;

// note that 'move' is a rust keyword, so when intended as variable 'mov' is used instead here
pub struct Move {
    from: i8,
    to: i8,
}

pub fn generate_moves(bit_board: BitBoard) -> Vec<Move> {
    let moves = Vec::<Move>::new();

    moves
}

pub fn make_move(bit_board: BitBoard, mov: Move) -> BitBoard {
    1
}

pub fn unmake_move(bit_board: BitBoard, mov: Move) -> BitBoard {
    1
}
