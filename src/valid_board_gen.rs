use crate::constants::{BitBoard, BOARD_HEIGHT, BOARD_WIDTH};

// algorithm from https://stackoverflow.com/a/2075867/5460583
pub fn generate_valid_boards() {
    // equals 12 ones in binary
    const START: i64 = 4096 - 1;

    // equals 42 choose 12, i.e. the number of ways to choose 12 elements (uisge pieces) from a set of 42 elements (board tiles)
    const MAX: i64 = 11058116888;

    let mut v = START;

    let mut j = 0;
    let mut k = 0;
    for i in 0..MAX {
        if is_connected(v, v.trailing_zeros() as i8) {
            k += 1;
        }

        let t = v | (v - 1);
        let w = (t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1));
        v = w;

        if i % 100000000 == 0 {
            println!("{} {}/{} ({}%) {:042b}", j, k, i, k as f32 / i as f32, w);
            j += 1;
        }
    }
}

pub fn is_connected(bit_board: i64, bit_index: i8) -> bool {
    let x = bit_index % BOARD_WIDTH;
    let y = bit_index / BOARD_WIDTH;

    flood_fill(bit_board, x, y) == 0
}

fn flood_fill(bit_board: BitBoard, x: i8, y: i8) -> BitBoard {
    let mut bb = bit_board;

    if !is_board_coord_set(bb, x, y) {
        return bit_board;
    }

    bb = unset_bit(bb, x, y);
    bb = flood_fill(bb, x - 1, y);
    bb = flood_fill(bb, x + 1, y);
    bb = flood_fill(bb, x, y - 1);
    bb = flood_fill(bb, x, y + 1);

    bb
}

fn unset_bit(bit_board: BitBoard, x: i8, y: i8) -> BitBoard {
    let mask = 1 << (BOARD_WIDTH * y + x);
    bit_board & !mask
}

fn is_board_bit_set(bit_board: i64, bit_index: i8) -> bool {
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
fn is_board_coord_set(bit_board: i64, x: i8, y: i8) -> bool {
    if x < 0 || x > BOARD_WIDTH || y < 0 || y > BOARD_HEIGHT {
        return false;
    }

    is_board_bit_set(bit_board, BOARD_WIDTH * y + x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_connected() {
        let x = make_board(
            "\
            0001000\
            0011100\
            0011111\
            0000010\
            0000010\
            0000010\
        ",
        );

        assert_eq!(is_connected(x, 1), true);
        assert_eq!(is_connected(x, 38), true);
        assert_eq!(is_connected(x, 0), false);


        let x = make_board(
            "\
            0001000\
            0011100\
            0011101\
            0000010\
            0000010\
            0000010\
        ",
        );

        assert_eq!(is_connected(x, 1), false);
        assert_eq!(is_connected(x, 38), false);
    }

    #[test]
    fn test_unset_bit() {
        let x = make_board(
            "\
            0001000\
            0011100\
            0011111\
            0000010\
            0000010\
            0000010\
        ",
        );
        assert_eq!(x, 0b000100000111000011111000001000000100000010);
        assert_eq!(
            unset_bit(x, 0, 0),
            0b000100000111000011111000001000000100000010
        );
        assert_eq!(
            unset_bit(x, 1, 0),
            0b000100000111000011111000001000000100000000
        );
        assert_eq!(
            unset_bit(x, 1, 1),
            0b000100000111000011111000001000000000000010
        );
        assert_eq!(
            unset_bit(x, 1, 2),
            0b000100000111000011111000000000000100000010
        );
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
        let x = make_board(
            "\
            0001000\
            0011100\
            0011111\
            0000010\
            0000010\
            0000010\
        ",
        );

        assert_eq!(is_board_coord_set(x, -10, 0), false);
        assert_eq!(is_board_coord_set(x, 10, 0), false);
        assert_eq!(is_board_coord_set(x, 0, -20), false);
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

    fn make_board(s: &str) -> BitBoard {
        i64::from_str_radix(s, 2).unwrap()
    }
}
