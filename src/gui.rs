use std::io::stdin;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::constants::COLOR_GREEN;
use crate::constants::COLOR_RED;
use crate::constants::COLOR_RESET;
use crate::move_gen::Move;
use crate::search::think;
use crate::state::GameState;

#[derive(PartialEq)]
pub enum GameMode {
    ComputerBlack,
    ComputerWhite,
    NoComputer,
}

pub struct Configuration {
    pub game_mode: GameMode,
    pub search_depth_max: u32,
}

pub fn start_gui(game_state: &mut GameState) {
    let game_mode: GameMode;
    let search_depth_max = get_search_depth_max();

    println!();

    if SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 2
        == 0
    {
        println!(r#"      __  ___               "#);
        println!(r#"     / / / (_)________ ____ "#);
        println!(r#"    / / / / / ___/ __ `/ _ \"#);
        println!(r#"   / /_/ / (__  ) /_/ /  __/"#);
        println!(r#"   \____/_/____/\__, /\___/ "#);
        println!(r#"               /____/       "#);
        println!(r#"                            "#);
    } else {
        println!(r#"   | |  | (_)               "#);
        println!(r#"   | |  | |_ ___  __ _  ___ "#);
        println!(r#"   | |  | | / __|/ _` |/ _ \"#);
        println!(r#"   | |__| | \__ \ (_| |  __/"#);
        println!(r#"    \____/|_|___/\__, |\___|"#);
        println!(r#"                  __/ |     "#);
        println!(r#"                 |___/      "#);
    }

    println!();
    println!("Configuration:");
    println!("    Computer search depth: {search_depth_max}");
    println!();
    println!("Select game mode:");
    println!("    b - play against computer as black");
    println!("    w - play against computer as white");
    println!("    h - play both sides");
    println!("    q - quit");
    loop {
        let mut buffer = String::new();
        let stdin = stdin();
        stdin.read_line(&mut buffer).unwrap();
        buffer.pop();

        match buffer.as_str() {
            "b" => {
                game_mode = GameMode::ComputerWhite;
                break;
            }
            "w" => {
                game_mode = GameMode::ComputerBlack;
                break;
            }
            "h" => {
                game_mode = GameMode::NoComputer;
                break;
            }
            "q" => return,
            _ => println!("Please enter one of the options: b/w/h/q"),
        }
    }

    let config = Configuration {
        game_mode,
        search_depth_max,
    };

    run_game(game_state, config);
}

fn run_game(game_state: &mut GameState, config: Configuration) {
    loop {
        println!("*******************************************************");
        println!("{game_state}");
        let moves = game_state.generate_moves();
        println!("{moves:?}");
        print_move_list_indices(&moves);

        //println!("{:?}", game_state.board);

        if is_computers_turn(game_state, &config.game_mode) {
            let mov = think(game_state, config.search_depth_max);
            println!("{COLOR_GREEN}computer moves {mov:?}{COLOR_RESET}");

            game_state.make_move(mov);
            continue;
        } else {
            println!("Select a move number, type 'a' to analyze position or 'q' to quit:");
        }

        let mut buffer = String::new();
        let stdin = stdin();
        stdin.read_line(&mut buffer).unwrap();

        buffer.pop();

        if let Ok(move_list_index) = buffer.parse::<usize>() {
            if move_list_index == 0 || move_list_index > moves.len() {
                println!("{COLOR_RED}invalid move number{COLOR_RESET}");
            } else {
                let mov = moves[move_list_index - 1];
                println!("{COLOR_GREEN}you move {mov:?}{COLOR_RESET}");

                game_state.make_move(mov);
            }
        } else if buffer == "a" {
            println!("{COLOR_GREEN}analyze position{COLOR_RESET} (quit with ctrl+c)");
            think(game_state, 100);
        } else if buffer == "q" {
            println!("{COLOR_GREEN}quit{COLOR_RESET}");
            break;
        } else {
            println!("{COLOR_RED}invalid input{COLOR_RESET}");
        }
    }
}

fn get_search_depth_max() -> u32 {
    if let Ok(sdm_str) = std::env::var("SEARCH_DEPTH_MAX") {
        if let Ok(sdm_num) = sdm_str.parse::<u32>() {
            return sdm_num;
        }
    };

    11
}

fn is_computers_turn(game_state: &GameState, game_mode: &GameMode) -> bool {
    if *game_mode == GameMode::ComputerWhite && game_state.is_active_player_white {
        return true;
    }

    if *game_mode == GameMode::ComputerBlack && !game_state.is_active_player_white {
        return true;
    }

    false
}

fn print_move_list_indices(moves: &[Move]) {
    let mut indices = String::new();

    for i in 0..moves.len() {
        indices.push_str(&format!("   {:>2}   ", i + 1));
    }

    println!("{indices}");
}
