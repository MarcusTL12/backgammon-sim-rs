use super::{GameState, MoveBuffer};

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

            let mut moves = MoveBuffer::new();

            let p = 1.0 / 36.0;
            for d1 in 1..=6 {
                for d2 in d1..=6 {
                    let p = if d1 == d2 { p } else { 2.0 * p };

                    if let Some((_, e)) =
                        self.get_best_move(turn, [d1, d2], depth, &mut moves)
                    {
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
        moves: &mut MoveBuffer,
    ) -> Option<(Self, f64)> {
        moves.generate(turn, *self, dice);

        let mut best = None;

        for new_state in moves.state_iterator() {
            let eval = new_state.get_brute_force_eval(!turn, depth - 1);

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
