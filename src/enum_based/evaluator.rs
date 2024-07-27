use std::collections::HashMap;

use arrayvec::ArrayVec;

use super::GameState;

impl GameState {
    fn get_net_dist(&self) -> i32 {
        let [light_dist, dark_dist] = self.get_tot_dist();

        (light_dist as i32) - (dark_dist as i32)
    }

    pub fn get_brute_force_eval(&self, turn: bool, depth: u32) -> f64 {
        if depth == 0 {
            self.get_net_dist() as f64
        } else {
            let mut eval = 0.0;

            let mut moves = Vec::new();
            let mut buf1 = Vec::new();
            let mut buf2 = Vec::new();

            let p = 1.0 / 36.0;
            for d1 in 1..=6 {
                for d2 in d1..=6 {
                    let p = if d1 == d2 { p } else { 2.0 * p };

                    if let Some((_, e)) = self.get_best_move(
                        turn,
                        [d1, d2],
                        depth,
                        &mut moves,
                        &mut buf1,
                        &mut buf2,
                    ) {
                        eval += p * e;
                    } else {
                        panic!("No legal moves!");
                    }
                }
            }

            eval
        }
    }

    pub fn get_best_move(
        &self,
        turn: bool,
        dice: [u8; 2],
        depth: u32,
        moves: &mut Vec<ArrayVec<[u8; 2], 4>>,
        move_buf: &mut Vec<Vec<[u8; 2]>>,
        seen_states_buf: &mut Vec<HashMap<GameState, ArrayVec<[u8; 2], 4>>>,
    ) -> Option<(ArrayVec<[u8; 2], 4>, f64)> {
        self.get_possible_moves_double(
            turn,
            dice,
            moves,
            move_buf,
            seen_states_buf,
        );

        let mut best_move = None;
        let mut best_eval = None;

        for ms in moves {
            let mut new_state = self.clone();
            for &m in &*ms {
                new_state.do_move(m);
            }
            let eval = new_state.get_brute_force_eval(!turn, depth - 1);

            if let Some(e) = &mut best_eval {
                if (*e < eval) ^ turn {
                    best_move = Some(ms.clone());
                    *e = eval;
                }
            } else {
                best_move = Some(ms.clone());
                best_eval = Some(eval);
            }
        }

        if let (Some(m), Some(e)) = (best_move, best_eval) {
            Some((m, e))
        } else {
            None
        }
    }
}
