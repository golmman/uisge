use crate::constants::{BitBoard, BoardIndex};
use crate::state::GameState;

// note that 'move' is a rust keyword, so when intended as variable 'mov' is used instead here
#[derive(Clone, Copy)]
pub struct Move {
    from: BoardIndex,
    to: BoardIndex,
}

//pub fn generate_king_moves(game_state: &GameState,

impl GameState {
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        let board = &self.board;

        let (kings, pawns) = self.get_active_pieces();

        for king in kings {
            self.append_king_moves(&mut moves, king);
        }

        moves
    }

    fn append_king_moves(&self, moves: &mut Vec<Move>, king: BoardIndex) {}

    pub fn make_move(&mut self, mov: Move) -> BitBoard {
        1
    }

    pub fn unmake_move(&mut self, mov: Move) -> BitBoard {
        1
    }
}
