use std::fmt::Debug;

use crate::bit_utils::{get_bit_indices, is_board_bit_set, jump_bit};
use crate::constants::{BitBoard, BoardIndex, JUMP_MOVES, BOARD_WIDTH};
use crate::state::GameState;
use crate::valid_board_gen::is_connected;

// note that 'move' is a rust keyword, so when intended as variable 'mov' is used instead here
#[derive(Clone, Copy)]
pub struct Move {
    from: BoardIndex,
    to: BoardIndex,
}

impl Move {
    pub fn new(from: BoardIndex, to: BoardIndex) -> Self {
        Self { from, to }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_file = ('a' as u8 + self.from % BOARD_WIDTH) as char;
        let from_rank = self.from / BOARD_WIDTH + 1;
        let to_file = ('a' as u8 + self.to % BOARD_WIDTH) as char;
        let to_rank = self.to / BOARD_WIDTH + 1;

        write!(f, "{}{}->{}{}", from_file, from_rank, to_file, to_rank)
    }
}

impl GameState {
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();

        let (kings, pawns) = self.get_active_pieces();

        for king in kings {
            self.append_jump_moves(&mut moves, king);
            self.append_king_moves(&mut moves, king);
        }

        for pawn in pawns {
            self.append_jump_moves(&mut moves, pawn);
        }

        moves
    }

    fn append_king_moves(&self, moves: &mut Vec<Move>, from: BoardIndex) {}

    fn append_jump_moves(&self, moves: &mut Vec<Move>, from: BoardIndex) {
        let jumps = JUMP_MOVES[from as usize];
        let allowed_jump_bits = jumps & !self.board.piece_bits;
        let allowed_jump_indices = get_bit_indices(allowed_jump_bits);

        for to in allowed_jump_indices {
            if is_between_occupied(self.board.piece_bits, from, to) {
                let moved_bit_board = jump_bit(self.board.piece_bits, from, to);
                if is_connected(moved_bit_board, to) {
                    moves.push(Move::new(from, to));
                }
            }
        }
    }

    pub fn make_move(&mut self, mov: Move) -> BitBoard {
        1
    }

    pub fn unmake_move(&mut self, mov: Move) -> BitBoard {
        1
    }
}

fn is_between_occupied(board: BitBoard, a1: BoardIndex, a2: BoardIndex) -> bool {
    is_board_bit_set(board, calculate_between_index(a1, a2))
}

fn calculate_between_index(a1: BoardIndex, a2: BoardIndex) -> BoardIndex {
    if a1 > a2 {
        return calculate_between_index(a2, a1);
    }

    let diff = a2 - a1;

    if diff == 2 {
        return a1 + 1;
    }

    if diff == 14 {
        return a1 + 7;
    }

    99
}
