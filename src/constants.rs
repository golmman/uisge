pub type BitBoard = u64;
pub type BoardIndex = u8;
pub type BoardIndexList = [BoardIndex; 8];

pub const BOARD_WIDTH: u8 = 7;
pub const BOARD_HEIGHT: u8 = 6;
pub const BOARD_TOTAL_PIECES: u8 = BOARD_HEIGHT * BOARD_WIDTH;

const fn make_board(bits: [u8; 42]) -> u64 {
    let mut board: u64 = 0;

    let mut i = 0;
    while i < 42 {
        if bits[i] != 0 {
            board |= 1 << i;
        };

        i += 1;
    }

    board
}

// TODO: generate this!!!!!!!

#[rustfmt::skip]
pub const KINGS: [u64; 2] = [
    make_board([
        0, 1, 0, 0, 0, 0, 0,
        1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ]),
    make_board([
        1, 0, 1, 0, 0, 0, 0,
        1, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ]),
];
