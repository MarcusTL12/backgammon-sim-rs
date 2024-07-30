use super::{GameState, SPECIAL_MOVE};

#[derive(Debug)]
pub struct MoveBuffer {
    pub dice: [u8; 2],
    pub turn: bool,
    pub state: GameState,
    pub single: [Vec<u8>; 2],
    pub double: [Vec<[u8; 2]>; 2],
    pub triple: Vec<[u8; 3]>,
    pub quadruple: Vec<[u8; 4]>,
}

impl MoveBuffer {
    pub fn new() -> Self {
        Self {
            dice: [0; 2],
            turn: false,
            state: GameState::new(),
            single: [const { Vec::new() }; 2],
            double: [const { Vec::new() }; 2],
            triple: Vec::new(),
            quadruple: Vec::new(),
        }
    }

    fn generate_double(&mut self) {
        for from1 in (0..24).chain([SPECIAL_MOVE]) {
            if let Ok(state1) =
                self.state.do_move(self.turn, from1, self.dice[0])
            {
                self.single[0].push(from1);

                for from2 in (0..24).chain([SPECIAL_MOVE]) {
                    if state1.do_move(self.turn, from2, self.dice[1]).is_ok() {
                        self.double[0].push([from1, from2]);
                    }
                }
            } else if let Ok(state1) =
                self.state.do_move(self.turn, from1, self.dice[1])
            {
                let from2 = if from1 == SPECIAL_MOVE {
                    if self.turn {
                        self.dice[1] - 1
                    } else {
                        24 - self.dice[1]
                    }
                } else if self.turn {
                    from1 + self.dice[1]
                } else {
                    from1 - self.dice[1]
                };

                if state1.do_move(self.turn, from2, self.dice[0]).is_ok() {
                    self.double[1].push([from1, from2]);
                }
            }

            if self.state.do_move(self.turn, from1, self.dice[1]).is_ok() {
                self.single[1].push(from1);
            }
        }
    }

    fn generate_quadruple(&mut self) {
        for from1 in (0..24).chain([SPECIAL_MOVE]) {
            let Ok(state1) = self.state.do_move(self.turn, from1, self.dice[0])
            else {
                continue;
            };
            self.single[0].push(from1);
            for from2 in (from1..24).chain([SPECIAL_MOVE]) {
                let Ok(state2) = state1.do_move(self.turn, from2, self.dice[0])
                else {
                    continue;
                };
                self.double[0].push([from1, from2]);
                for from3 in (from2..24).chain([SPECIAL_MOVE]) {
                    let Ok(state3) =
                        state2.do_move(self.turn, from3, self.dice[0])
                    else {
                        continue;
                    };
                    self.triple.push([from1, from2, from3]);
                    for from4 in (from3..24).chain([SPECIAL_MOVE]) {
                        if state3
                            .do_move(self.turn, from4, self.dice[0])
                            .is_ok()
                        {
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
        self.single[1].clear();
        self.double[0].clear();
        self.double[1].clear();
        self.triple.clear();
        self.quadruple.clear();

        dice.sort();

        self.dice = dice;
        self.turn = turn;
        self.state = state;

        if dice[0] == dice[1] {
            self.generate_quadruple();
        } else {
            self.generate_double();
        }
    }

    pub fn state_iterator(&self) -> StateIterator {
        use StateIterator::*;
        if !self.quadruple.is_empty() {
            Quadruple(self.state, self.turn, self.dice[0], &self.quadruple, 0)
        } else if !self.triple.is_empty() {
            Triple(self.state, self.turn, self.dice[0], &self.triple, 0)
        } else if !self.double[0].is_empty() || !self.double[1].is_empty() {
            Double(
                self.state,
                self.turn,
                self.dice,
                [&self.double[0], &self.double[1]],
                true,
                0,
            )
        } else if !self.single[0].is_empty() || !self.single[1].is_empty() {
            Single(
                self.state,
                self.turn,
                self.dice,
                [&self.single[0], &self.single[1]],
                true,
                0,
            )
        } else {
            NoMoves(self.state, true)
        }
    }
}

#[derive(Debug)]
pub enum StateIterator<'a> {
    NoMoves(GameState, bool),
    Single(GameState, bool, [u8; 2], [&'a [u8]; 2], bool, usize),
    Double(GameState, bool, [u8; 2], [&'a [[u8; 2]]; 2], bool, usize),
    Triple(GameState, bool, u8, &'a [[u8; 3]], usize),
    Quadruple(GameState, bool, u8, &'a [[u8; 4]], usize),
}

impl<'a> Iterator for StateIterator<'a> {
    type Item = GameState;

    fn next(&mut self) -> Option<GameState> {
        use StateIterator::*;
        match self {
            NoMoves(state, should_return) => should_return.then(|| {
                *should_return = false;
                *state
            }),
            Single(state, turn, dice, moves, first, i) => {
                if *first {
                    if let Some(&from) = moves[0].get(*i) {
                        *i += 1;

                        Some(state.do_move(*turn, from, dice[0]).unwrap())
                    } else if let Some(&from) = moves[1].first() {
                        *i = 1;
                        *first = false;

                        Some(state.do_move(*turn, from, dice[1]).unwrap())
                    } else {
                        None
                    }
                } else if let Some(&from) = moves[1].get(*i) {
                    *i += 1;

                    Some(state.do_move(*turn, from, dice[1]).unwrap())
                } else {
                    None
                }
            }
            Double(state, turn, dice, moves, forwards, i) => {
                if *forwards {
                    if let Some(&[from1, from2]) = moves[0].get(*i) {
                        *i += 1;

                        Some(
                            state
                                .do_move(*turn, from1, dice[0])
                                .unwrap()
                                .do_move(*turn, from2, dice[1])
                                .unwrap(),
                        )
                    } else if let Some(&[from1, from2]) = moves[1].first() {
                        *i = 1;
                        *forwards = false;

                        Some(
                            state
                                .do_move(*turn, from1, dice[1])
                                .unwrap()
                                .do_move(*turn, from2, dice[0])
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                } else if let Some(&[from1, from2]) = moves[1].get(*i) {
                    *i += 1;

                    Some(
                        state
                            .do_move(*turn, from1, dice[1])
                            .unwrap()
                            .do_move(*turn, from2, dice[0])
                            .unwrap(),
                    )
                } else {
                    None
                }
            }
            Triple(state, turn, die, moves, i) => moves.get(*i).map(|&froms| {
                *i += 1;

                let mut state = *state;

                for from in froms {
                    state = state.do_move(*turn, from, *die).unwrap();
                }

                state
            }),
            Quadruple(state, turn, die, moves, i) => {
                moves.get(*i).map(|&froms| {
                    *i += 1;

                    let mut state = *state;

                    for from in froms {
                        state = state.do_move(*turn, from, *die).unwrap();
                    }

                    state
                })
            }
        }
    }
}
