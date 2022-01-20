use std::fmt::Display;

use crate::constants::{BitBoard, BoardIndex, BOARD_HEIGHT, BOARD_TOTAL_PIECES, BOARD_WIDTH};
use crate::piece_list::PieceList;
use crate::valid_board_gen::make_board;

pub struct GameState {
    pub board: Board,
    pub is_active_player_white: bool,
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        let is_active_player_white = true;

        Self {
            board,
            is_active_player_white,
        }
    }

    pub fn get_active_pieces(&self) -> (PieceList, PieceList) {
        match self.is_active_player_white {
            true => (self.board.white_kings, self.board.white_pawns),
            false => (self.board.black_kings, self.board.black_pawns),
        }
    }
}

// 00 01 02 03 04 05 06
// 07 08 09 10 11 12 13
// 14 15 16 17 18 19 20
// 21 22 23 24 25 26 27
// 28 29 30 31 32 33 34
// 35 36 37 38 39 40 41
pub struct Board {
    pub piece_bits: BitBoard,

    pub black_kings: PieceList,
    pub black_pawns: PieceList,
    pub white_kings: PieceList,
    pub white_pawns: PieceList,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const RESET: &str = "\x1b[0m";
        const WHITE_ON_MAGENTA: &str = "\x1b[37;45m";
        const BLACK_ON_MAGENTA: &str = "\x1b[30;45m";

        let mut text = String::new();

        text.push_str(&format!(
            "{}{}{}\n",
            BLACK_ON_MAGENTA, "   a b c d e f g   ", RESET
        ));

        let mut piece =
            vec![format!("{}{}{}", BLACK_ON_MAGENTA, "- ", RESET); BOARD_TOTAL_PIECES as usize];

        for i in self.black_kings {
            piece[i as usize] = format!("{}{}{}", BLACK_ON_MAGENTA, "W ", RESET);
        }
        for i in self.black_pawns {
            piece[i as usize] = format!("{}{}{}", BLACK_ON_MAGENTA, "o ", RESET);
        }
        for i in self.white_kings {
            piece[i as usize] = format!("{}{}{}", WHITE_ON_MAGENTA, "W ", RESET);
        }
        for i in self.white_pawns {
            piece[i as usize] = format!("{}{}{}", WHITE_ON_MAGENTA, "o ", RESET);
        }

        for i in 0..BOARD_TOTAL_PIECES {
            if i % BOARD_WIDTH == 0 {
                text.push_str(&format!(
                    "{} {} {}",
                    BLACK_ON_MAGENTA,
                    i / BOARD_WIDTH + 1,
                    RESET
                ));
            }

            text.push_str(&piece[i as usize]);

            if (i + 1) % BOARD_WIDTH == 0 {
                text.push_str(&format!(
                    "{}{} {}\n",
                    BLACK_ON_MAGENTA,
                    i / BOARD_WIDTH + 1,
                    RESET
                ));
            }
        }

        text.push_str(&format!(
            "{}{}{}\n",
            BLACK_ON_MAGENTA, "   a b c d e f g   ", RESET
        ));

        write!(f, "{}", text)
    }
}

impl Board {
    pub fn new() -> Self {
        let piece_bits = make_board(
            "\
            0000000\
            0011000\
            0111100\
            0011110\
            0001100\
            0000000\
            ",
        );

        let black_kings = PieceList::new();
        let black_pawns = PieceList::from(vec![23, 24, 25, 26, 31, 32]);

        let white_kings = PieceList::new();
        let white_pawns = PieceList::from(vec![9, 10, 15, 16, 17, 18]);

        Self {
            piece_bits,
            black_kings,
            black_pawns,
            white_kings,
            white_pawns,
        }
    }
}
