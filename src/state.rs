use std::fmt::Display;

use crate::constants::{BOARD_HEIGHT, BOARD_TOTAL_PIECES, BOARD_WIDTH};

enum Piece {
    BlackKing,
    BlackPawn,
    WhiteKing,
    WhitePawn,

    None,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // \033[41m
        let displayed_text = match self {
            Piece::BlackKing => "\x1b[30mW",
            Piece::BlackPawn => "\x1b[30mo",
            Piece::WhiteKing => "\x1b[33mW",
            Piece::WhitePawn => "\x1b[33mo",
            Piece::None => "\x1b[30m-",
        };

        //write!(f, "\x1b[47m {}\x1b[0m", displayed_text)
        write!(f, " {}", displayed_text)
    }
}

// 00 01 02 03 04 05 06
// 07 08 09 10 11 12 13
// 14 15 16 17 18 19 20
// 21 22 23 24 25 26 27
// 28 29 30 31 32 33 34
// 35 36 37 38 39 40 41

pub struct Board {
    pieces: Vec<Piece>, // TODO: slice instead of Vec?
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        text.push_str("\x1b[47m");
        text.push_str("\x1b[30m +---------------+ ");
        text.push_str("\x1b[0m");
        text.push_str("\n");

        for y in 0..BOARD_HEIGHT {
            text.push_str("\x1b[47m");
            text.push_str("\x1b[30m |");
            text.push_str("\x1b[0m");

            for x in 0..BOARD_WIDTH {
                let i = (BOARD_WIDTH * y + x) as usize;
                text.push_str("\x1b[47m");
                text.push_str(&format!("{}", self.pieces[i]));
                text.push_str("\x1b[0m");
            }

            text.push_str("\x1b[47m");
            text.push_str("\x1b[30m | ");
            text.push_str("\x1b[0m");
            text.push_str("\n");
        }

        text.push_str("\x1b[47m");
        text.push_str("\x1b[30m +---------------+ ");
        text.push_str("\x1b[0m");
        text.push_str("\n");

        write!(f, "{}", text)
    }
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = Vec::<Piece>::new();

        for i in 0..BOARD_TOTAL_PIECES {
            pieces.push(Piece::None);
        }

        pieces[9] = Piece::WhitePawn;
        pieces[10] = Piece::WhitePawn;
        pieces[15] = Piece::WhitePawn;
        pieces[16] = Piece::WhitePawn;
        pieces[17] = Piece::WhitePawn;
        pieces[18] = Piece::WhitePawn;

        pieces[24] = Piece::BlackPawn;
        pieces[25] = Piece::BlackPawn;
        pieces[23] = Piece::BlackPawn;
        pieces[26] = Piece::BlackPawn;
        pieces[31] = Piece::BlackPawn;
        pieces[32] = Piece::BlackPawn;

        Self { pieces }
    }
}
