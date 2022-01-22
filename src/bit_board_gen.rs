use crate::bit_utils::print_bit_board;
use crate::bit_utils::set_board_bit;
use crate::constants::BOARD_HEIGHT;
use crate::constants::BOARD_WIDTH;

pub fn print_bit_board_code() {
    print_king_moves();
    println!();
    print_jump_moves();
}

fn print_king_moves() {
    println!("pub const KING_MOVES: [u64; 42] = [");

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let mut bits = 0;

            bits = set_board_bit(bits, x.wrapping_sub(1), y.wrapping_sub(1));
            bits = set_board_bit(bits, x, y.wrapping_sub(1));
            bits = set_board_bit(bits, x + 1, y.wrapping_sub(1));

            bits = set_board_bit(bits, x.wrapping_sub(1), y);
            bits = set_board_bit(bits, x + 1, y);

            bits = set_board_bit(bits, x.wrapping_sub(1), y + 1);
            bits = set_board_bit(bits, x, y + 1);
            bits = set_board_bit(bits, x + 1, y + 1);

            println!("    0b{bits:042b},");

            //println!("-------");
            //print_bit_board(bits);
        }
    }

    println!("];");
}

fn print_jump_moves() {
    println!("pub const JUMP_MOVES: [u64; 42] = [");

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let mut bits = 0;

            bits = set_board_bit(bits, x, y.wrapping_sub(2));

            bits = set_board_bit(bits, x.wrapping_sub(2), y);
            bits = set_board_bit(bits, x + 2, y);

            bits = set_board_bit(bits, x, y + 2);

            println!("    0b{bits:042b},");

            //println!("-------");
            //print_bit_board(bits);
        }
    }

    println!("];");
}
