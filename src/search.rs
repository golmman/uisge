use crate::constants::SCORE_KING_COUNT;
use crate::constants::SCORE_MAX;
use crate::constants::SCORE_MIN;
use crate::move_gen::Move;
use crate::state::GameState;

struct PVLine {
    len: usize,
    moves: [Move; 100],
}

pub fn think(game_state: &GameState, max_depth: u32) -> Move {
    let mut m = Move::new(0, 0);

    for depth in 1..max_depth {
        let (score, mov) = pvs(game_state, SCORE_MIN, SCORE_MAX, depth);
        m = mov;
        println!("{depth:>3} | {score:>5} | {mov:?}");
    }

    m
}

//pub fn start_pvs(game_state: &GameState, depth: u32) -> (i32

pub fn pvs(game_state: &GameState, alpha: i32, beta: i32, depth: u32) -> (i32, Move) {
    let mut a = alpha;
    let b = beta;
    let mut score: i32;
    let mut best_move = Move::new(0, 0);

    // TODO: move sorting with principal variation
    let moves = game_state.generate_moves();

    if depth == 0 || moves.is_empty() {
        return (evaluate(game_state), best_move);
    }

    for (i, mov) in moves.into_iter().enumerate() {
        let mut game_state_move = game_state.clone();
        game_state_move.make_move(mov);

        if i == 0 {
            score = -pvs(&game_state_move, -b, -a, depth - 1).0;
        } else {
            score = -pvs(&game_state_move, -a - 1, -a, depth - 1).0;
            if a < score && score < b {
                score = -pvs(&game_state_move, -b, -score, depth - 1).0;
            }
        }

        if score > a {
            a = score;
            best_move = mov;
        }

        if a >= b {
            break;
        }
    }

    (a, best_move)
}

pub fn evaluate(game_state: &GameState) -> i32 {
    let white_king_score = SCORE_KING_COUNT[game_state.board.white_kings.len()];
    let black_king_score = SCORE_KING_COUNT[game_state.board.black_kings.len()];
    let move_count_score = game_state.move_count as i32;

    // TODO
    // why seems "+ move_count_score" to work here?
    // possibly because a higher move count is good in a losing position but
    // bad in a winning position, and black is currently the computer, so winning
    let score = white_king_score - black_king_score + move_count_score;

    if game_state.is_active_player_white {
        score
    } else {
        -score
    }
}
