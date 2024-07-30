use std::cmp::Ordering;

use move_generator::MoveBuffer;

mod display;
mod move_generator;
// mod evaluator;

const SPECIAL_MOVE: u8 = 99;

// Light: (Positive, forward, true)
// Dark:  (Negative, backward, false)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
    tiles: [i8; 24],
    captured: [u8; 2],
    finished: [u8; 2],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            tiles: [0; 24],
            captured: [0, 0],
            finished: [0, 0],
        }
    }

    pub fn new_with_default_setup() -> Self {
        let mut state = Self::new();

        state.tiles[0] = 2;
        state.tiles[5] = -5;
        state.tiles[7] = -3;
        state.tiles[11] = 5;
        state.tiles[12] = -5;
        state.tiles[16] = 3;
        state.tiles[18] = 5;
        state.tiles[23] = -2;

        state
    }

    pub fn get_tot_dist(&self) -> [u32; 2] {
        let mut ans = self.captured.map(|x| x as u32 * 25);

        for (i, &t) in self.tiles.iter().enumerate() {
            match t.cmp(&0) {
                Ordering::Equal => {}
                Ordering::Greater => ans[0] += t as u32 * (24 - i as u32),
                Ordering::Less => ans[1] += -t as u32 * (i as u32 + 1),
            }
        }

        ans
    }

    pub fn is_all_home(&self, player: bool) -> bool {
        if self.captured[(!player) as usize] != 0 {
            return false;
        }

        if player {
            for i in 0..18 {
                if self.tiles[i] > 0 {
                    return false;
                }
            }
        } else {
            for i in 6..24 {
                if self.tiles[i] < 0 {
                    return false;
                }
            }
        }

        true
    }

    pub fn do_move(
        mut self,
        turn: bool,
        from: u8,
        n: u8,
    ) -> Result<Self, &'static str> {
        match from {
            SPECIAL_MOVE => {
                if self.captured[(!turn) as usize] > 0 {
                    let target_spot =
                        if turn { n - 1 } else { 24 - n } as usize;

                    match (turn, self.tiles[target_spot]) {
                        (true, 0..=127) => {
                            self.captured[0] -= 1;
                            self.tiles[target_spot] += 1;
                            Ok(self)
                        }
                        (false, -128..=0) => {
                            self.captured[1] -= 1;
                            self.tiles[target_spot] -= 1;
                            Ok(self)
                        }
                        (true, -1) => {
                            self.captured[0] -= 1;
                            self.captured[1] += 1;
                            self.tiles[target_spot] = 1;
                            Ok(self)
                        }
                        (false, 1) => {
                            self.captured[1] -= 1;
                            self.captured[0] += 1;
                            self.tiles[target_spot] = -1;
                            Ok(self)
                        }
                        (true, -128..=-2) | (false, 2..=127) => {
                            Err("Target spot occupied") // Checked
                        }
                    }
                } else {
                    Err("No captures pieces") // Checked
                }
            }
            0..=23 => {
                if turn && self.tiles[from as usize] <= 0
                    || !turn && self.tiles[from as usize] >= 0
                {
                    Err("No movable pieces to move") // Checked
                } else {
                    let target_spot = from as isize
                        + if turn { n as isize } else { -(n as isize) };

                    match target_spot {
                        0..=23 => {
                            let target_spot = target_spot as usize;

                            match (turn, self.tiles[target_spot]) {
                                (true, 0..=127) => {
                                    self.tiles[from as usize] -= 1;
                                    self.tiles[target_spot] += 1;
                                    Ok(self)
                                }
                                (false, -128..=0) => {
                                    self.tiles[from as usize] += 1;
                                    self.tiles[target_spot] -= 1;
                                    Ok(self)
                                }
                                (true, -1) => {
                                    self.tiles[from as usize] -= 1;
                                    self.captured[1] += 1;
                                    self.tiles[target_spot] = 1;
                                    Ok(self)
                                }
                                (false, 1) => {
                                    self.tiles[from as usize] += 1;
                                    self.captured[0] += 1;
                                    self.tiles[target_spot] = -1;
                                    Ok(self)
                                }
                                (true, -128..=-2) | (false, 2..=127) => {
                                    Err("Target spot occupied") // Checked
                                }
                            }
                        }
                        _ => {
                            if self.is_all_home(turn) {
                                match (turn, target_spot) {
                                    (true, 24) => {
                                        self.tiles[from as usize] -= 1;
                                        self.finished[0] += 1;
                                        Ok(self)
                                    }
                                    (false, -1) => {
                                        self.tiles[from as usize] += 1;
                                        self.finished[1] += 1;
                                        Ok(self)
                                    }
                                    (true, _) => {
                                        if (18..from)
                                            .any(|i| self.tiles[i as usize] > 0)
                                        {
                                            Err("Full moves available") // Checked
                                        } else {
                                            self.tiles[from as usize] -= 1;
                                            self.finished[0] += 1;
                                            Ok(self)
                                        }
                                    }
                                    (false, _) => {
                                        if (from + 1..6)
                                            .any(|i| self.tiles[i as usize] < 0)
                                        {
                                            Err("Full moves available") // Checked
                                        } else {
                                            self.tiles[from as usize] -= 1;
                                            self.finished[0] += 1;
                                            Ok(self)
                                        }
                                    }
                                }
                            } else {
                                Err("All pieces are not home") // Checked
                            }
                        }
                    }
                }
            }
            _ => Err("Illegal space"), // Checked
        }
    }
}

pub fn _test1() -> Result<(), &'static str> {
    let state = GameState::new_with_default_setup();

    let r = state.do_move(true, 100, 6);
    println!("{r:?}");

    let r = state.do_move(true, 0, 5);
    println!("{r:?}");

    let state = state.do_move(true, 0, 6)?.do_move(true, 0, 6)?;

    println!("{state}");

    let r = state.do_move(true, 0, 6);
    println!("{r:?}");

    let r = state.do_move(true, 18, 6);
    println!("{r:?}");

    let r = state.do_move(true, SPECIAL_MOVE, 3);
    println!("{r:?}");

    let state = state.do_move(true, 6, 2)?.do_move(false, 7, 1)?;
    println!("{state}");

    let r = state.do_move(true, SPECIAL_MOVE, 6);
    println!("{r:?}");

    let state = state.do_move(true, SPECIAL_MOVE, 4)?;
    println!("{state}");

    let state = state
        .do_move(true, 3, 18)?
        .do_move(true, 8, 12)?
        .do_move(true, 11, 10)?
        .do_move(true, 11, 10)?
        .do_move(true, 11, 10)?
        .do_move(true, 11, 10)?
        .do_move(true, 11, 10)?
        .do_move(true, 16, 3)?
        .do_move(true, 16, 3)?
        .do_move(true, 16, 3)?;
    println!("{state}");

    let state = state.do_move(true, 19, 5)?;
    println!("{state}");

    let r = state.do_move(true, 19, 6);
    println!("{r:?}");

    let state = state
        .do_move(false, 23, 20)?
        .do_move(false, 23, 20)?
        .do_move(false, 12, 12)?
        .do_move(false, 12, 12)?
        .do_move(false, 12, 12)?
        .do_move(false, 12, 12)?
        .do_move(false, 12, 12)?
        .do_move(false, 7, 3)?
        .do_move(false, 7, 3)?
        .do_move(false, 6, 3)?
        .do_move(false, 4, 5)?;
    println!("{state}");

    let r = state.do_move(false, 4, 6);
    println!("{r:?}");

    Ok(())
}

pub fn _test2() -> Result<(), &'static str> {
    let state = GameState::new_with_default_setup();

    println!("{state}");

    let mut moves = MoveBuffer::new();

    moves.generate(true, state, [1, 1]);

    println!("{moves:?}");

    println!("Moves with only dice: {}", moves.dice[0]);

    for &s in &moves.single[0] {
        let new_state = state.do_move(true, s, moves.dice[0]).unwrap();
        println!("{new_state}");
    }

    println!("Moves with only dice: {}", moves.dice[1]);

    for &s in &moves.single[1] {
        let new_state = state.do_move(true, s, moves.dice[1]).unwrap();
        println!("{new_state}");
    }

    println!("Moves with dice: {:?}", moves.dice);

    for &[s1, s2] in &moves.double[0] {
        let new_state = state.do_move(true, s1, moves.dice[0])?.do_move(
            true,
            s2,
            moves.dice[1],
        )?;
        println!("{new_state}");
    }

    println!("Reverse moves with dice: {:?}", moves.dice);

    for &[s1, s2] in &moves.double[1] {
        let new_state = state.do_move(true, s1, moves.dice[1])?.do_move(
            true,
            s2,
            moves.dice[0],
        )?;
        println!("{new_state}");
    }

    println!("Triple moves with die: {:?}", moves.dice[0]);

    for &[s1, s2, s3] in &moves.triple {
        let new_state = state
            .do_move(true, s1, moves.dice[0])?
            .do_move(true, s2, moves.dice[0])?
            .do_move(true, s3, moves.dice[0])?;
        println!("{new_state}");
    }

    println!("Quadruple moves with die: {:?}", moves.dice[0]);

    for &[s1, s2, s3, s4] in &moves.quadruple {
        let new_state = state
            .do_move(true, s1, moves.dice[0])?
            .do_move(true, s2, moves.dice[0])?
            .do_move(true, s3, moves.dice[0])?
            .do_move(true, s4, moves.dice[0])?;
        println!("{new_state}");
    }

    Ok(())
}

pub fn _test3() {
    let state = GameState::new_with_default_setup();

    println!("{state}");

    let mut moves = MoveBuffer::new();

    moves.generate(true, state, [5, 5]);

    for new_state in moves.state_iterator() {
        println!("{new_state}");
    }
}
