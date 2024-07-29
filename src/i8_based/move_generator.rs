use super::{GameState, SPECIAL_MOVE};

#[derive(Debug)]
pub struct MoveBuffer {
    pub dice: [u8; 2],
    pub single: [Vec<u8>; 2],
    pub double: [Vec<[u8; 2]>; 2],
    pub triple: Vec<[u8; 3]>,
    pub quadruple: Vec<[u8; 4]>,
}

impl MoveBuffer {
    pub fn new() -> Self {
        Self {
            dice: [0; 2],
            single: [const { Vec::new() }; 2],
            double: [const { Vec::new() }; 2],
            triple: Vec::new(),
            quadruple: Vec::new(),
        }
    }

    fn generate_double(&mut self, turn: bool, state: GameState) {
        for from1 in (0..24).chain([SPECIAL_MOVE]) {
            if let Ok(state1) = state.do_move(turn, from1, self.dice[0]) {
                self.single[0].push(from1);

                for from2 in (0..24).chain([SPECIAL_MOVE]) {
                    if state1.do_move(turn, from2, self.dice[1]).is_ok() {
                        self.double[0].push([from1, from2]);
                    }
                }
            } else if let Ok(state1) = state.do_move(turn, from1, self.dice[1])
            {
                let from2 = if from1 == SPECIAL_MOVE {
                    if turn {
                        self.dice[1] - 1
                    } else {
                        24 - self.dice[1]
                    }
                } else if turn {
                    from1 + self.dice[1]
                } else {
                    from1 - self.dice[1]
                };

                if state1.do_move(turn, from2, self.dice[0]).is_ok() {
                    self.double[1].push([from1, from2]);
                }
            }

            if state.do_move(turn, from1, self.dice[1]).is_ok() {
                self.single[1].push(from1);
            }
        }
    }

    fn generate_quadruple(&mut self, turn: bool, state: GameState) {
        for from1 in (0..24).chain([SPECIAL_MOVE]) {
            let Ok(state1) = state.do_move(turn, from1, self.dice[0]) else {
                continue;
            };
            self.single[0].push(from1);
            for from2 in (from1..24).chain([SPECIAL_MOVE]) {
                let Ok(state2) = state1.do_move(turn, from2, self.dice[0])
                else {
                    continue;
                };
                self.double[0].push([from1, from2]);
                for from3 in (from2..24).chain([SPECIAL_MOVE]) {
                    let Ok(state3) = state2.do_move(turn, from3, self.dice[0])
                    else {
                        continue;
                    };
                    self.triple.push([from1, from2, from3]);
                    for from4 in (from3..24).chain([SPECIAL_MOVE]) {
                        if state3.do_move(turn, from4, self.dice[0]).is_ok() {
                            self.quadruple.push([from1, from2, from3, from4]);
                        }
                    }
                }
            }
        }
    }

    pub fn generate(
        &mut self,
        turn: bool,
        state: GameState,
        mut dice: [u8; 2],
    ) {
        self.single[0].clear();
        self.double[0].clear();
        self.double[1].clear();
        self.triple.clear();
        self.quadruple.clear();

        dice.sort();

        self.dice = dice;

        if dice[0] == dice[1] {
            self.generate_quadruple(turn, state);
        } else {
            self.generate_double(turn, state);
        }
    }
}
