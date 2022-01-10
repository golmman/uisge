use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};

// algorithm from https://stackoverflow.com/a/2075867/5460583
pub fn generate_valid_boards() {
    // equals 12 ones in binary
    const START: i64 = 4096 - 1;

    // equals 42 choose 12, i.e. the number of ways to choose 12 elements (uisge pieces) from a set of 42 elements (board tiles)
    const MAX: i64 = 11058116888;

    let mut v = START;
    for i in 0..MAX {
        let t = v | (v - 1);
        let w = (t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1));
        v = w;

        if i % 1000000000 == 0 {
            println!("{:042b}", w);
        }
    }
}

// algorithm from https://en.wikipedia.org/wiki/Flood_fill#Span_Filling
pub fn check_connected(bit_board: i64, bit_index: u8) -> bool {
    println!("{:042b}", bit_board);
    println!("{:042b}", bit_board & 1 << bit_index);
    println!("{}", is_board_bit_set(bit_board, bit_index));
    false
}

fn is_board_bit_set(bit_board: i64, bit_index: u8) -> bool {
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
fn is_board_coord_set(bit_board: i64, x: u8, y: u8) -> bool {
    if x < 0 || x > BOARD_WIDTH || y < 0 || y > BOARD_HEIGHT {
        return false;
    }

    is_board_bit_set(bit_board, BOARD_WIDTH * y + x)
}

#[cfg(test)]
mod test {
    use crate::constants::BitBoard;

    use super::*;

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
