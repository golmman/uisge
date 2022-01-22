use std::fmt::Debug;

use crate::bit_utils::get_bit_indices;
use crate::bit_utils::is_board_bit_set;
use crate::bit_utils::jump_bit;
use crate::constants::BitBoard;
use crate::constants::BoardIndex;
use crate::constants::BOARD_WIDTH;
use crate::constants::COLOR_RESET;
use crate::constants::COLOR_YELLOW;
use crate::constants::JUMP_MOVES;
use crate::constants::KING_MOVES;
use crate::state::GameState;
use crate::valid_board_gen::is_connected;

// note that 'move' is a rust keyword, so when intended as variable 'mov' is used instead here
#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    from: BoardIndex,
    to: BoardIndex,
}

impl Move {
    pub fn new(from: BoardIndex, to: BoardIndex) -> Self {
        Self { from, to }
    }

    pub fn is_jump(&self) -> bool {
        let from = self.from as i8;
        let to = self.to as i8;

        let diff = from - to;

        match diff {
            2 => true,
            -2 => true,
            14 => true,
            -14 => true,
            _ => false,
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_file = ('a' as u8 + self.from % BOARD_WIDTH) as char;
        let from_rank = self.from / BOARD_WIDTH + 1;
        let to_file = ('a' as u8 + self.to % BOARD_WIDTH) as char;
        let to_rank = self.to / BOARD_WIDTH + 1;

        if self.is_jump() {
            write!(f, "{from_file}{from_rank}->{to_file}{to_rank}")
        } else {
            write!(
                f,
                "{COLOR_YELLOW}{from_file}{from_rank}->{to_file}{to_rank}{COLOR_RESET}"
            )
        }
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

    fn append_king_moves(&self, moves: &mut Vec<Move>, from: BoardIndex) {
        let king_moves = KING_MOVES[from as usize];
        let allowed_jump_bits = king_moves & !self.board.piece_bits;
        let allowed_jump_indices = get_bit_indices(allowed_jump_bits);

        for to in allowed_jump_indices {
            let moved_bit_board = jump_bit(self.board.piece_bits, from, to);
            if !is_connected(moved_bit_board, to) {
                continue;
            }

            moves.push(Move::new(from, to));
        }
    }

    fn append_jump_moves(&self, moves: &mut Vec<Move>, from: BoardIndex) {
        let jump_moves = JUMP_MOVES[from as usize];
        let allowed_jump_bits = jump_moves & !self.board.piece_bits;
        let allowed_jump_indices = get_bit_indices(allowed_jump_bits);

        for to in allowed_jump_indices {
            if !is_between_occupied(self.board.piece_bits, from, to) {
                continue;
            }

            let moved_bit_board = jump_bit(self.board.piece_bits, from, to);
            if !is_connected(moved_bit_board, to) {
                continue;
            }

            moves.push(Move::new(from, to));
        }
    }

    pub fn make_move(&mut self, mov: Move) {
        let (mut kings, mut pawns) = self.get_active_pieces();

        self.board.piece_bits = jump_bit(self.board.piece_bits, mov.from, mov.to);

        if mov.is_jump() {
            if kings.find_and_remove(mov.from) {
                pawns.push_front(mov.to);
            } else if pawns.find_and_remove(mov.from) {
                kings.push_front(mov.to);
            } else {
                panic!("couldn't find jump move piece in king or pawn lists");
            }
        } else {
            if kings.find_and_remove(mov.from) {
                kings.push_front(mov.to);
            } else {
                panic!("couldn't find king move piece in king list");
            }
        }

        self.set_active_pieces(kings, pawns);

        self.is_active_player_white = !self.is_active_player_white;
        self.move_count += 1;
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

    panic!();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::BoardIndex;

    #[test]
    fn test_generate_moves_from_initial_position() {
        let game_state = GameState::new();

        let moves = game_state.generate_moves();

        assert_eq!(
            moves,
            vec![
                Move::new(9, 11),
                Move::new(10, 8),
                Move::new(17, 3),
                Move::new(17, 19),
            ]
        );
    }
}
