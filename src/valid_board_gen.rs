use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use crate::constants::{BitBoard, BoardIndex, BOARD_HEIGHT, BOARD_WIDTH};

// algorithm from https://stackoverflow.com/a/2075867/5460583
pub fn generate_valid_boards() {
    let mut file = File::create("connected_boards_test.dat").unwrap();
    let mut connected_boards = Vec::<BitBoard>::new();

    // equals 12 ones in binary
    const START: BitBoard = 4096 - 1;

    // equals 42 choose 12, i.e. the number of ways to choose 12 elements (uisge pieces) from a set of 42 elements (board tiles)
    const MAX: BitBoard = 11058116888;

    let mut v = START;

    let mut j = 0;
    let mut k = 0;
    for i in 0..MAX {
        if is_connected(v, v.trailing_zeros() as BoardIndex) {
            connected_boards.push(v);
            k += 1;
        }

        let t = (v | (v - 1)) as i64;
        let w = (t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1));
        v = w as u64;

        if i % 100000000 == 0 {
            println!("{} {}/{} ({}%) {:042b}", j, k, i, k as f32 / i as f32, w);
            j += 1;

            //if (j == 2) {
            //    for ppp in 0..10 {
            //        println!("{:042b}", connected_boards[ppp + 1000]);
            //        print_bit_board(connected_boards[ppp + 1000]);
            //        println!("");
            //    }
            //    file.write_all(&boards_to_bytes(&connected_boards));
            //    return;
            //}
        }
    }

    file.write_all(&boards_to_bytes(&connected_boards));
}

pub fn make_board(s: &str) -> BitBoard {
    BitBoard::from_str_radix(s, 2).unwrap()
}

pub fn is_connected(bit_board: BitBoard, bit_index: BoardIndex) -> bool {
    let x = bit_index % BOARD_WIDTH;
    let y = bit_index / BOARD_WIDTH;

    flood_fill(bit_board, x, y) == 0
}

fn flood_fill(bit_board: BitBoard, x: BoardIndex, y: BoardIndex) -> BitBoard {
    let mut bb = bit_board;

    if !is_board_coord_set(bb, x, y) {
        return bit_board;
    }

    bb = unset_bit(bb, x, y);
    bb = flood_fill(bb, x.wrapping_sub(1), y);
    bb = flood_fill(bb, x + 1, y);
    bb = flood_fill(bb, x, y.wrapping_sub(1));
    bb = flood_fill(bb, x, y + 1);

    bb
}

fn unset_bit(bit_board: BitBoard, x: BoardIndex, y: BoardIndex) -> BitBoard {
    let mask = 1 << (BOARD_WIDTH * y + x);
    bit_board & !mask
}

fn is_board_bit_set(bit_board: BitBoard, bit_index: BoardIndex) -> bool {
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
fn is_board_coord_set(bit_board: BitBoard, x: BoardIndex, y: BoardIndex) -> bool {
    if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
        return false;
    }

    is_board_bit_set(bit_board, BOARD_WIDTH * y + x)
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

pub fn bytes_to_boards(bytes: &Vec<u8>) -> Vec<BitBoard> {
    let bit_boards_len = bytes.len() / 8; // if not aligned, some data is lost
    let mut bit_boards = Vec::with_capacity(bit_boards_len);

    for i in 0..bit_boards_len {
        let bit_board = unsafe {
            std::mem::transmute::<[u8; 8], BitBoard>([
                bytes[i * 8 + 7],
                bytes[i * 8 + 6],
                bytes[i * 8 + 5],
                bytes[i * 8 + 4],
                bytes[i * 8 + 3],
                bytes[i * 8 + 2],
                bytes[i * 8 + 1],
                bytes[i * 8],
            ])
        };

        bit_boards.push(bit_board);
    }

    bit_boards
}

pub fn boards_to_bytes(bit_boards: &Vec<BitBoard>) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8 * bit_boards.len());

    for bit_board in bit_boards {
        bytes.extend(&bit_board.to_be_bytes());
    }

    bytes
}

pub fn read_valid_boards() -> HashSet<BitBoard> {
    let mut file = File::open("connected_boards.dat").unwrap();

    let mut d = Vec::<u8>::new();
    file.read_to_end(&mut d).unwrap();

    let mut data = bytes_to_boards(&d);

    data.iter().cloned().collect()
}

// TODO
// called from this spot:
// is_connected: 15509
// contains: 4809
//
// if you copy this function to main.rs
// is_connected: 2520
// contains: 4777
//
// WHY??
pub fn benchmark_valid_board_hashing() {
    let mut connected_boards = read_valid_boards();

    let x: BitBoard = 0b000100000111000011111000001000000100000010;

    let now = Instant::now();
    for i in 0..100000000 {
        let a = if now.elapsed().as_millis() % 2 == 0 {
            1
        } else {
            0
        };
        is_connected(x + a, 1);
    }
    println!("is_connected: {}", now.elapsed().as_millis());

    let now = Instant::now();
    for i in 0..100000000 {
        let a = if now.elapsed().as_millis() % 2 == 0 {
            1
        } else {
            0
        };
        connected_boards.contains(&(x + a));
    }
    println!("contains: {}", now.elapsed().as_millis());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_boards_to_bytes() {
        let mut x = Vec::<BitBoard>::new();
        x.push(5345345345234243);
        x.push(143954759751381111);

        let a = boards_to_bytes(&x);
        let b = bytes_to_boards(&a);

        assert_eq!(a.len(), 16);
        assert_eq!(b.len(), 2);
        assert_eq!(x, b);
    }

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

        let x = make_board(
            "\
            0000000\
            0000000\
            0000000\
            0000101\
            1111110\
            1001011\
        ",
        );

        assert_eq!(is_connected(x, 0), false);
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
