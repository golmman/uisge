use crate::constants::{BitBoard, BoardIndex, BOARD_HEIGHT, BOARD_WIDTH};

pub const fn make_board(bits: [[u8; 7]; 6]) -> BitBoard {
    let mut board: BitBoard = 0;

    let mut y = 0;
    while y < 6 {
        let mut x = 0;
        while x < 7 {
            if bits[y][x] != 0 {
                board |= 1 << 7 * (5 - y) + (6 - x);
            }
            x += 1;
        }
        y += 1;
    }

    board
}

pub fn print_bit_board(bit_board: BitBoard) {
    let mut line = format!("{:042b}", bit_board);
    for i in 0..6 {
        let mut l = line.clone();
        let (left, right) = l.split_at_mut(7);
        line = right.to_string();
        println!("{}", left);
    }
}

pub fn get_bit_indices(i: u64) -> Vec<u8> {
    let mut indices = Vec::new();

    let mut j = i;

    loop {
        let index = j.trailing_zeros();
        if index == 64 {
            break;
        }

        indices.push(index as u8);

        j ^= 1 << index;
    }

    indices
}

pub fn is_board_bit_set(bit_board: BitBoard, bit_index: BoardIndex) -> bool {
    bit_board & 1 << bit_index != 0
}

// Note that the coordination system for a binary string looks like this
//            ^
//   0001000\ | y
//   0011100\ |
//   0011111\ |
//   0000010\ |
//   0000010\ |
//   0000010  |
// <----------+
//  x
pub fn is_board_coord_set(bit_board: BitBoard, x: BoardIndex, y: BoardIndex) -> bool {
    if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
        return false;
    }

    is_board_bit_set(bit_board, BOARD_WIDTH * y + x)
}

pub fn set_board_bit(bit_board: BitBoard, x: BoardIndex, y: BoardIndex) -> BitBoard {
    if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
        return bit_board;
    }

    bit_board | 1 << (BOARD_WIDTH * y + x)
}

pub fn jump_bit(bit_board: BitBoard, from: BoardIndex, to: BoardIndex) -> BitBoard {
    let from_mask = 1 << from;
    (bit_board | 1 << to) & !from_mask
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_board_bit() {
        let x = make_board([
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
        ]);

        let y = set_board_bit(x, 10, 5);
        assert_eq!(x, y);

        let y = set_board_bit(x, 0, 50);
        assert_eq!(x, y);

        let y = set_board_bit(x, 6, 4);
        let x = make_board([
            [0, 0, 0, 1, 0, 0, 0],
            [1, 0, 1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
        ]);
        assert_eq!(x, y);
    }

    #[test]
    fn test_is_board_bit_set() {
        let x = 0b0000000111001;
        assert_eq!(is_board_bit_set(x, 0), true);
        assert_eq!(is_board_bit_set(x, 1), false);
        assert_eq!(is_board_bit_set(x, 2), false);
        assert_eq!(is_board_bit_set(x, 3), true);
        assert_eq!(is_board_bit_set(x, 4), true);
        assert_eq!(is_board_bit_set(x, 5), true);
        assert_eq!(is_board_bit_set(x, 6), false);
    }

    #[test]
    fn test_is_board_coord_set() {
        let x = make_board([
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0],
        ]);

        assert_eq!(is_board_coord_set(x, 0u8.wrapping_sub(10), 0), false);
        assert_eq!(is_board_coord_set(x, 10, 0), false);
        assert_eq!(is_board_coord_set(x, 0, 0u8.wrapping_sub(20)), false);
        assert_eq!(is_board_coord_set(x, 0, 20), false);

        assert_eq!(is_board_coord_set(x, 0, 0), false);
        assert_eq!(is_board_coord_set(x, 1, 0), true);
        assert_eq!(is_board_coord_set(x, 2, 0), false);
        assert_eq!(is_board_coord_set(x, 3, 0), false);
        assert_eq!(is_board_coord_set(x, 4, 0), false);
        assert_eq!(is_board_coord_set(x, 5, 0), false);
        assert_eq!(is_board_coord_set(x, 6, 0), false);

        assert_eq!(is_board_coord_set(x, 0, 1), false);
        assert_eq!(is_board_coord_set(x, 1, 1), true);
        assert_eq!(is_board_coord_set(x, 2, 1), false);
        assert_eq!(is_board_coord_set(x, 3, 1), false);
        assert_eq!(is_board_coord_set(x, 4, 1), false);
        assert_eq!(is_board_coord_set(x, 5, 1), false);
        assert_eq!(is_board_coord_set(x, 6, 1), false);

        assert_eq!(is_board_coord_set(x, 0, 2), false);
        assert_eq!(is_board_coord_set(x, 1, 2), true);
        assert_eq!(is_board_coord_set(x, 2, 2), false);
        assert_eq!(is_board_coord_set(x, 3, 2), false);
        assert_eq!(is_board_coord_set(x, 4, 2), false);
        assert_eq!(is_board_coord_set(x, 5, 2), false);
        assert_eq!(is_board_coord_set(x, 6, 2), false);

        assert_eq!(is_board_coord_set(x, 0, 3), true);
        assert_eq!(is_board_coord_set(x, 1, 3), true);
        assert_eq!(is_board_coord_set(x, 2, 3), true);
        assert_eq!(is_board_coord_set(x, 3, 3), true);
        assert_eq!(is_board_coord_set(x, 4, 3), true);
        assert_eq!(is_board_coord_set(x, 5, 3), false);
        assert_eq!(is_board_coord_set(x, 6, 3), false);

        assert_eq!(is_board_coord_set(x, 0, 4), false);
        assert_eq!(is_board_coord_set(x, 1, 4), false);
        assert_eq!(is_board_coord_set(x, 2, 4), true);
        assert_eq!(is_board_coord_set(x, 3, 4), true);
        assert_eq!(is_board_coord_set(x, 4, 4), true);
        assert_eq!(is_board_coord_set(x, 5, 4), false);
        assert_eq!(is_board_coord_set(x, 6, 4), false);

        assert_eq!(is_board_coord_set(x, 0, 5), false);
        assert_eq!(is_board_coord_set(x, 1, 5), false);
        assert_eq!(is_board_coord_set(x, 2, 5), false);
        assert_eq!(is_board_coord_set(x, 3, 5), true);
        assert_eq!(is_board_coord_set(x, 4, 5), false);
        assert_eq!(is_board_coord_set(x, 5, 5), false);
        assert_eq!(is_board_coord_set(x, 6, 5), false);
    }
}
