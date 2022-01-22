use crate::constants::COLOR_GREEN;
use crate::constants::COLOR_RED;
use crate::constants::COLOR_RESET;
use crate::move_gen::Move;
use crate::search::think;
use crate::state::GameState;
use std::io::stdin;

pub fn start_gui(game_state: &mut GameState) {
    loop {
        println!("*******************************************************");
        println!("{}", game_state);
        let moves = game_state.generate_moves();
        println!("{:?}", moves);
        print_move_list_indices(&moves);
        println!("select a move number or type q for quit:");

        //println!("{:?}", game_state.board);

        if !game_state.is_active_player_white {
            let mov = think(game_state, 9);
            game_state.make_move(mov);
            continue;
        }

        let mut buffer = String::new();
        let mut stdin = stdin();
        stdin.read_line(&mut buffer).unwrap();

        buffer.pop();

        if let Ok(move_list_index) = buffer.parse::<usize>() {
            if move_list_index <= 0 || move_list_index > moves.len() {
                println!("{COLOR_RED}invalid move number{COLOR_RESET}");
            } else {
                let mov = moves[move_list_index - 1];
                println!("{COLOR_GREEN}moving {mov:?}{COLOR_RESET}");

                game_state.make_move(mov);
            }
        } else if buffer == "q" {
            println!("{COLOR_GREEN}quit{COLOR_RESET}");
            break;
        } else {
            println!("{COLOR_RED}invalid input{COLOR_RESET}");
        }
    }
}

pub fn print_move_list_indices(moves: &Vec<Move>) {
    let mut indices = String::new();

    for i in 0..moves.len() {
        indices.push_str(&format!("   {:>2}   ", i + 1));
    }

    println!("{indices}");
}
