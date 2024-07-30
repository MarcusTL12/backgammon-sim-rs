use super::{GameState, MoveBuffer};

impl GameState {
    fn get_net_dist(&self) -> i32 {
        let [light_dist, dark_dist] = self.get_tot_dist();

        (light_dist as i32) - (dark_dist as i32)
    }
}

pub struct Evaluator {
    move_buffers: Vec<MoveBuffer>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { move_buffers: Vec::new() }
    }

    pub fn get_brute_force_eval(
        &mut self,
        state: GameState,
        turn: bool,
        depth: u32,
    ) -> f64 {
        if depth == 0 {
            state.get_net_dist() as f64
        } else {
            let mut eval = 0.0;

            let mut moves = self.move_buffers.pop().unwrap_or_default();

            let p = 1.0 / 36.0;
            for d1 in 1..=6 {
                for d2 in d1..=6 {
                    let p = if d1 == d2 { p } else { 2.0 * p };

                    if let Some((_, e)) =
                        self.get_best_move(state, turn, [d1, d2], depth, &mut moves)
                    {
                        eval += p * e;
                    } else {
                        panic!("No legal moves!");
                    }
                }
            }

            self.move_buffers.push(moves);

            eval
        }
    }

    pub fn get_best_move(
        &mut self,
        state: GameState,
        turn: bool,
        dice: [u8; 2],
        depth: u32,
        moves: &mut MoveBuffer,
    ) -> Option<(GameState, f64)> {
        moves.generate(turn, state, dice);

        let mut best = None;

        for new_state in moves.state_iterator() {
            let eval = self.get_brute_force_eval(new_state, !turn, depth - 1);

            if let Some((s, e)) = &mut best {
                if (*e < eval) ^ turn {
                    *s = new_state;
                    *e = eval;
                }
            } else {
                best = Some((new_state, eval))
            }
        }

        best
    }
}
