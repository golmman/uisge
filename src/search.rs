use std::fmt::Display;
use std::time::Instant;

use crate::constants::SCORE_KING_COUNT;
use crate::constants::SCORE_MAX;
use crate::constants::SCORE_MIN;
use crate::move_gen::Move;
use crate::state::GameState;

pub struct PVLine {
    moves: Vec<Move>,
}

impl PVLine {
    fn new() -> Self {
        Self { moves: Vec::new() }
    }

    fn from_pv_line_tail(pv_line: &PVLine) -> Self {
        let mut moves = Vec::new();

        for i in 1..pv_line.moves.len() {
            moves.push(pv_line.moves[i]);
        }

        Self { moves }
    }

    fn update(&mut self, mov: Move, pv_line: &mut PVLine) {
        self.moves.clear();
        self.moves.push(mov);
        self.moves.append(&mut pv_line.moves);
    }
}

impl Display for PVLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.moves)
    }
}

pub fn think(game_state: &GameState, max_depth: u32) -> Move {
    let start_instant = Instant::now();

    let mut pv_line = PVLine::new();

    println!("      time | depth |      score | principal variation");
    println!(" ----------|-------|------------|---------------------------");
    for depth in 1..max_depth + 1 {
        let score_int = pvs(game_state, SCORE_MIN, SCORE_MAX, depth, &mut pv_line);
        let score = score_int as f32 / 1000f32;
        let elapsed = start_instant.elapsed().as_millis() as f32 / 1000f32;

        println!("{elapsed:>10.3} | {depth:>5} | {score:>10.3} | {pv_line}");
    }

    pv_line.moves[0]
}

pub fn pvs(game_state: &GameState, alpha: i32, beta: i32, depth: u32, pv_line: &mut PVLine) -> i32 {
    let mut new_pv_line = PVLine::from_pv_line_tail(pv_line); //PVLine::new();
    let mut a = alpha;
    let b = beta;
    let mut score: i32;

    if depth == 0 {
        return evaluate(game_state);
    }

    let mut moves = game_state.generate_moves();

    if moves.is_empty() {
        return SCORE_MIN;
    }

    if !pv_line.moves.is_empty() {
        swap_move_to_front(&mut moves, pv_line.moves[0]);
    }

    for (i, mov) in moves.into_iter().enumerate() {
        let mut game_state_move = game_state.clone();
        game_state_move.make_move(mov);

        if i == 0 {
            score = -pvs(&game_state_move, -b, -a, depth - 1, &mut new_pv_line);
        } else {
            if depth > 2 && !mov.is_jump() {
                // late move reduction
                score = -pvs(&game_state_move, -a - 1, -a, depth - 2, &mut new_pv_line);
            } else {
                score = -pvs(&game_state_move, -a - 1, -a, depth - 1, &mut new_pv_line);
            }

            if a < score && score < b {
                score = -pvs(&game_state_move, -b, -score, depth - 1, &mut new_pv_line);
            }
        }

        if score > a {
            a = score;
            pv_line.update(mov, &mut new_pv_line);
        }

        if a >= b {
            break;
        }
    }

    a
}

pub fn evaluate(game_state: &GameState) -> i32 {
    let white_king_score = SCORE_KING_COUNT[game_state.board.white_kings.len()];
    let black_king_score = SCORE_KING_COUNT[game_state.board.black_kings.len()];
    let move_count_score = game_state.move_count as i32;

    let mut score = white_king_score - black_king_score;

    // In a winning position high we are penalizing longer games,
    // in a losing position we reward longer games.
    if score > 0 {
        score -= move_count_score;
    } else {
        score += move_count_score;
    }

    if game_state.is_active_player_white {
        score
    } else {
        -score
    }
}

fn swap_move_to_front(moves: &mut [Move], mov: Move) {
    for i in 0..moves.len() {
        if moves[i] == mov {
            moves.swap(0, i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_pv_line() {
        let mut a = PVLine::new();
        let mut b = PVLine::new();
        let mut c = PVLine::new();
        let mut d = PVLine::new();

        b.update(Move::new(0, 1), &mut a);
        c.update(Move::new(1, 2), &mut b);
        d.update(Move::new(2, 3), &mut c);

        assert_eq!(
            d.moves,
            vec![Move::new(2, 3), Move::new(1, 2), Move::new(0, 1)]
        );
    }

    #[test]
    fn test_swap_move_to_front() {
        let mut moves = vec![
            Move::new(0, 1),
            Move::new(1, 2),
            Move::new(2, 3),
            Move::new(3, 4),
            Move::new(4, 5),
        ];

        swap_move_to_front(&mut moves, Move::new(3, 4));

        assert_eq!(
            moves,
            vec![
                Move::new(3, 4),
                Move::new(1, 2),
                Move::new(2, 3),
                Move::new(0, 1),
                Move::new(4, 5),
            ],
        );

        swap_move_to_front(&mut moves, Move::new(33, 4));

        assert_eq!(
            moves,
            vec![
                Move::new(3, 4),
                Move::new(1, 2),
                Move::new(2, 3),
                Move::new(0, 1),
                Move::new(4, 5),
            ],
        );
    }
}
