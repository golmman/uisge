use std::fmt::Display;

use crate::constants::{BitBoard, BOARD_HEIGHT, BOARD_TOTAL_PIECES, BOARD_WIDTH};
use crate::valid_board_gen::make_board;

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

pub struct Board2 {
    piece_bits: BitBoard,

    black_kings: Vec<i8>,
    black_pawns: Vec<i8>,
    white_kings: Vec<i8>,
    white_pawns: Vec<i8>,
}

impl Display for Board2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const RESET: &str = "\x1b[0m";
        const WHITE_ON_MAGENTA: &str = "\x1b[37;45m";
        const BLACK_ON_MAGENTA: &str = "\x1b[30;45m";

        let mut text = String::new();

        text.push_str(&format!(
            "{}{}{}\n",
            BLACK_ON_MAGENTA, " ***************** ", RESET
        ));

        let mut piece =
            vec![format!("{}{}{}", BLACK_ON_MAGENTA, "- ", RESET); BOARD_TOTAL_PIECES as usize];
        for i in &self.black_kings {
            piece[*i as usize] = format!("{}{}{}", BLACK_ON_MAGENTA, "W ", RESET);
        }
        for i in &self.black_pawns {
            piece[*i as usize] = format!("{}{}{}", BLACK_ON_MAGENTA, "o ", RESET);
        }
        for i in &self.white_kings {
            piece[*i as usize] = format!("{}{}{}", WHITE_ON_MAGENTA, "W ", RESET);
        }
        for i in &self.white_pawns {
            piece[*i as usize] = format!("{}{}{}", WHITE_ON_MAGENTA, "o ", RESET);
        }

        for i in 0..BOARD_TOTAL_PIECES {
            if i % BOARD_WIDTH == 0 {
                text.push_str(&format!("{}{}{}", BLACK_ON_MAGENTA, " * ", RESET));
            }

            text.push_str(&piece[i as usize]);

            if (i + 1) % BOARD_WIDTH == 0 {
                text.push_str(&format!("{}{}{}\n", BLACK_ON_MAGENTA, "* ", RESET));
            }
        }

        text.push_str(&format!(
            "{}{}{}\n",
            BLACK_ON_MAGENTA, " ***************** ", RESET
        ));

        write!(f, "{}", text)
    }
}

impl Board2 {
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

        let black_kings = Vec::new();
        let black_pawns = vec![23, 24, 25, 26, 31, 32];

        let white_kings = Vec::new();
        let white_pawns = vec![9, 10, 15, 16, 17, 18];

        Self {
            piece_bits,
            black_kings,
            black_pawns,
            white_kings,
            white_pawns,
        }
    }
}

pub struct Board {
    pieces: Vec<Piece>, // TODO: slice instead of Vec?

    black_kings: BitBoard,
    black_pawns: BitBoard,
    white_kings: BitBoard,
    white_pawns: BitBoard,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FG_YELLOW: &str = "\x1b[33m";
        const FG_BLACK: &str = "\x1b[30m";
        const BG_GRAY: &str = "\x1b[47m";
        const RESET: &str = "\x1b[0m";

        const WHITE_ON_MAGENTA: &str = "\x1b[37;45m";
        const BLACK_ON_MAGENTA: &str = "\x1b[30;45m";

        let mut text = String::new();

        text.push_str(&format!(
            "{}{}{}{}\n",
            BG_GRAY, FG_YELLOW, "ufffffff", RESET
        ));
        text.push_str(&format!("{}{}{}\n", WHITE_ON_MAGENTA, "ufffffff", RESET));
        text.push_str(&format!("{}{}{}\n", BLACK_ON_MAGENTA, "ufffffff", RESET));
        text.push_str("fasdfsadfads\n");

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

        let black_kings = 0;
        let black_pawns = make_board(
            "\
            0000000\
            0000000\
            0000000\
            0011110\
            0001100\
            0000000\
            ",
        );

        let white_kings = 0;
        let white_pawns = make_board(
            "\
            0000000\
            0011000\
            0111100\
            0000000\
            0000000\
            0000000\
            ",
        );

        Self {
            pieces,
            black_kings,
            black_pawns,
            white_kings,
            white_pawns,
        }
    }
}
