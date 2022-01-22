use crate::constants::{SCORE_MIN, SCORE_KING_COUNT, SCORE_MAX};
use crate::move_gen::Move;
use crate::state::GameState;

pub fn think(game_state: GameState, depth: u32) -> Move{
    let mut m = Move::new(0, 0);

    for d in 1..depth {
        let (score, mov) = nega_scout(&game_state, SCORE_MIN, SCORE_MAX, d);
        m = mov;
        println!("{d:>3} | {score:>5} | {mov:?}");
    }

    m
}

pub fn nega_scout(game_state: &GameState, alpha: i32, beta: i32, depth: u32) -> (i32, Move) {
    let mut a = alpha;
    let mut b = beta;
    let mut t: i32;
    let mut i = 0;

    let mut best_score = SCORE_MIN;
    let mut best_move = Move::new(0, 0);

    if depth == 0 {
        return (evaluate(game_state), best_move);
    }

    let moves = game_state.generate_moves();

    for mov in moves {
        let mut game_state_move = game_state.clone();
        game_state_move.make_move(mov);
        t = -nega_scout(&game_state_move, -b, -a, depth - 1).0;

        if t > a && t < beta && i > 1 {
            t = -nega_scout(&game_state_move, -beta, -t, depth - 1).0;
        }

        if best_score < t {
            best_score = t;
            best_move = mov;
        }

        a = a.max(t);

        if a >= beta {
            return (a, best_move);
        }

        b = a + 1;

        i += 1;
    }

    return (a, best_move);
}

pub fn evaluate(game_state: &GameState) -> i32 {
    let white_king_score = SCORE_KING_COUNT[game_state.board.white_kings.len()];
    let black_king_score = SCORE_KING_COUNT[game_state.board.black_kings.len()];

    if game_state.is_active_player_white {
        return white_king_score - black_king_score;
    } else {
        return black_king_score - white_king_score;
    }
}

//pub fn evaluate(game_state: &GameState) -> i32 {
//    let moves = game_state.generate_moves();
//
//    if moves.len() == 0 {
//        return SCORE_MIN;
//    }
//
//    let kings = game_state.get_active_kings();
//
//    (kings.len() + moves.len()) as i32
//}
