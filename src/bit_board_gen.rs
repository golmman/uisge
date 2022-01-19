use crate::bit_utils::{set_board_bit, print_bit_board};
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};

pub fn print_bit_board_code() {
    print_king_moves();
    println!();
    print_jump_moves();
}

fn print_king_moves() {
    println!("pub const KING_MOVES: [u64; 42] = [");

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let mut b = 0;

            b = set_board_bit(b, x.wrapping_sub(1), y.wrapping_sub(1));
            b = set_board_bit(b, x, y.wrapping_sub(1));
            b = set_board_bit(b, x + 1, y.wrapping_sub(1));

            b = set_board_bit(b, x.wrapping_sub(1), y);
            b = set_board_bit(b, x + 1, y);

            b = set_board_bit(b, x.wrapping_sub(1), y + 1);
            b = set_board_bit(b, x, y + 1);
            b = set_board_bit(b, x + 1, y + 1);

            println!("{}", format!("    0b{:042b},", b));

            //println!("-------");
            //print_bit_board(b);
        }
    }

    println!("];");
}

fn print_jump_moves() {
    println!("pub const JUMP_MOVES: [u64; 42] = [");

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let mut b = 0;

            b = set_board_bit(b, x, y.wrapping_sub(2));

            b = set_board_bit(b, x.wrapping_sub(2), y);
            b = set_board_bit(b, x + 2, y);

            b = set_board_bit(b, x, y + 2);

            println!("{}", format!("    0b{:042b},", b));

            //println!("-------");
            //print_bit_board(b);
        }
    }

    println!("];");
}
