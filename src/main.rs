mod game_display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Light(u8),
    Dark(u8),
}

use std::collections::HashMap;

use arrayvec::ArrayVec;
use Tile::*;

const SPECIAL_MOVE: u8 = 99;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    tiles: [Tile; 24],
    captured: [u8; 2],
    finished: [u8; 2],
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            tiles: [Empty; 24],
            captured: [0, 0],
            finished: [0, 0],
        }
    }

    pub fn new_with_default_setup() -> Self {
        let mut state = Self::new();

        state.tiles[0] = Light(2);
        state.tiles[5] = Dark(5);
        state.tiles[7] = Dark(3);
        state.tiles[11] = Light(5);
        state.tiles[12] = Dark(5);
        state.tiles[16] = Light(3);
        state.tiles[18] = Light(5);
        state.tiles[23] = Dark(2);

        state
    }

    pub fn is_all_home(&self) -> [bool; 2] {
        let mut home = self.captured.map(|x| x == 0);

        for t in &self.tiles[..6] {
            if let Light(_) = t {
                home[1] = false;
            }
        }

        for t in &self.tiles[6..18] {
            match t {
                Empty => {}
                Light(_) => home[0] = false,
                Dark(_) => home[1] = false,
            }
        }

        for t in &self.tiles[18..] {
            if let Dark(_) = t {
                home[0] = false;
            }
        }

        home
    }

    pub fn get_tot_dist(&self) -> [u32; 2] {
        let mut ans = self.captured.map(|x| x as u32 * 25);

        for (i, &t) in self.tiles.iter().enumerate() {
            match t {
                Empty => {}
                Light(n) => ans[0] += n as u32 * (24 - i as u32),
                Dark(n) => ans[1] += n as u32 * (i as u32 + 1),
            }
        }

        ans
    }

    // Returns true if moves are wasteful
    pub fn get_possible_moves(
        &self,
        turn: bool, // false => Light, true => Dark
        die: u8,
        moves: &mut Vec<[u8; 2]>,
    ) -> bool {
        let mut wasteful = false;

        moves.clear();

        let turn_ind = if turn { 1 } else { 0 };

        let move_dir = if turn { -1 } else { 1 };

        if self.captured[turn_ind] == 0 {
            for i in 0..24 {
                if let (Light(_), false) | (Dark(_), true) =
                    (self.tiles[i], turn)
                {
                    if let (Some(Empty), _)
                    | (Some(Light(_)), false)
                    | (Some(Dark(1)), false)
                    | (Some(Dark(_)), true)
                    | (Some(Light(1)), true) = (
                        self.tiles.get(
                            (i as isize + die as isize * move_dir) as usize,
                        ),
                        turn,
                    ) {
                        moves.push([
                            i as u8,
                            (i as isize + die as isize * move_dir) as u8,
                        ]);
                    }
                }
            }

            if self.is_all_home()[turn_ind] {
                let die_pos = if turn {
                    die as usize
                } else {
                    23 - die as usize
                };

                if let (Light(_), false) | (Dark(_), true) =
                    (self.tiles[die_pos], turn)
                {
                    moves.push([die_pos as u8, SPECIAL_MOVE]);
                }

                // At this point if no moves has been pushed,
                // we can check for 'wasteful' moves.

                if moves.is_empty() {
                    let mut i = die_pos as isize;
                    while let Some(t) = self.tiles.get(i as usize) {
                        if let (Light(_), false) | (Dark(_), true) = (t, turn) {
                            wasteful = true;
                            moves.push([i as u8, SPECIAL_MOVE]);
                            break;
                        }

                        i += move_dir;
                    }
                }
            }
        } else {
            let off = turn_ind * 18;
            for i in off..off + 6 {
                if let (Empty, _) | (Light(_), false) | (Dark(_), true) =
                    (self.tiles[i], turn)
                {
                    moves.push([SPECIAL_MOVE, i as u8]);
                }
            }
        }

        wasteful
    }

    pub fn do_move(&mut self, [from, to]: [u8; 2]) {
        match [from as usize, to as usize] {
            [f, 99] => match self.tiles[f] {
                Light(n) => {
                    self.tiles[f] = if n == 1 { Empty } else { Light(n - 1) };
                    self.finished[0] += 1;
                }
                Dark(n) => {
                    self.tiles[f] = if n == 1 { Empty } else { Dark(n - 1) };
                    self.finished[1] += 1;
                }
                Empty => panic!("Trying to move from empty tile {from}!"),
            },
            [99, t] => match t {
                0..=5 => match (self.captured[0], self.tiles[t]) {
                    (0, _) => panic!("No captured light pieces!"),
                    (_, Empty) => {
                        self.captured[0] -= 1;
                        self.tiles[t] = Light(1);
                    }
                    (_, Light(n)) => {
                        self.captured[0] -= 1;
                        self.tiles[t] = Light(n + 1);
                    }
                    (_, Dark(1)) => {
                        self.captured[0] -= 1;
                        self.tiles[t] = Light(1);
                        self.captured[1] += 1;
                    }
                    _ => panic!("Illegal move '{from} -> {to}'!"),
                },
                18..=23 => match (self.captured[1], self.tiles[t]) {
                    (0, _) => panic!("No captured dark pieces!"),
                    (_, Empty) => {
                        self.captured[1] -= 1;
                        self.tiles[t] = Dark(1);
                    }
                    (_, Dark(n)) => {
                        self.captured[1] -= 1;
                        self.tiles[t] = Dark(n + 1);
                    }
                    (_, Light(1)) => {
                        self.captured[1] -= 1;
                        self.tiles[t] = Dark(1);
                        self.captured[0] += 1;
                    }
                    _ => panic!("Illegal move '{from} -> {to}'!"),
                },
                _ => {
                    panic!("Trying to put captured piece in illegal position!");
                }
            },
            [f, t] => match [self.tiles[f], self.tiles[t]] {
                [Light(n), Empty] => {
                    self.tiles[f] = if n == 1 { Empty } else { Light(n - 1) };
                    self.tiles[t] = Light(1);
                }
                [Dark(n), Empty] => {
                    self.tiles[f] = if n == 1 { Empty } else { Dark(n - 1) };
                    self.tiles[t] = Dark(1);
                }
                [Light(n), Light(m)] => {
                    self.tiles[f] = if n == 1 { Empty } else { Light(n - 1) };
                    self.tiles[t] = Light(m + 1);
                }
                [Dark(n), Dark(m)] => {
                    self.tiles[f] = if n == 1 { Empty } else { Dark(n - 1) };
                    self.tiles[t] = Dark(m + 1);
                }
                [Light(n), Dark(1)] => {
                    self.tiles[f] = if n == 1 { Empty } else { Light(n - 1) };
                    self.tiles[t] = Light(1);
                    self.captured[1] += 1;
                }
                [Dark(n), Light(1)] => {
                    self.tiles[f] = if n == 1 { Empty } else { Dark(n - 1) };
                    self.tiles[t] = Dark(1);
                    self.captured[0] += 1;
                }
                [Empty, _] => panic!("Trying to move from empty tile {from}!"),
                _ => panic!("Illegal move '{from} -> {to}'!"),
            },
        }
    }

    pub fn get_possible_moves_double(
        &self,
        turn: bool,
        mut dice: [u8; 2],
        moves: &mut Vec<ArrayVec<[u8; 2], 4>>,
        move_buf: &mut Vec<Vec<[u8; 2]>>,
        seen_states_buf: &mut Vec<HashMap<GameState, ArrayVec<[u8; 2], 4>>>,
    ) {
        moves.clear();
        dice.sort();
        let dice = dice;

        if dice[0] != dice[1] {
            let mut buf1 = move_buf.pop().unwrap_or_default();
            let mut buf2 = move_buf.pop().unwrap_or_default();

            let wasteful1 = self.get_possible_moves(turn, dice[0], &mut buf1);

            // u for unused
            // w for wasteful
            let mut seen_ord = seen_states_buf.pop().unwrap_or_default();
            let mut seen_1w = seen_states_buf.pop().unwrap_or_default();
            let mut seen_2w = seen_states_buf.pop().unwrap_or_default();
            let mut seen_1u = seen_states_buf.pop().unwrap_or_default();
            let mut seen_1u1w = seen_states_buf.pop().unwrap_or_default();

            seen_ord.clear();
            seen_1w.clear();
            seen_2w.clear();
            seen_1u.clear();
            seen_1u1w.clear();

            if !buf1.is_empty() {
                for &m1 in &buf1 {
                    let mut newstate = self.clone();
                    newstate.do_move(m1);

                    if wasteful1 {
                        &mut seen_1u1w
                    } else {
                        &mut seen_1u
                    }
                    .insert(
                        newstate.clone(),
                        (&[m1] as &[_]).try_into().unwrap(),
                    );

                    let wasteful2 =
                        newstate.get_possible_moves(turn, dice[1], &mut buf2);

                    match [wasteful1, wasteful2] {
                        [false, false] => &mut seen_ord,
                        [true, false] | [false, true] => &mut seen_1w,
                        [true, true] => &mut seen_2w,
                    }
                    .extend(buf2.drain(..).map(|m2| {
                        let mut newstate2 = newstate.clone();
                        newstate2.do_move(m2);
                        (newstate2, (&[m1, m2] as &[_]).try_into().unwrap())
                    }));
                }
            }

            seen_states_buf.push(seen_1u);
            seen_states_buf.push(seen_2w);
            seen_states_buf.push(seen_1w);
            seen_states_buf.push(seen_ord);
            move_buf.push(buf2);
            move_buf.push(buf1);
        } else {
            // TODO: figure out how to deal with four equal dice.
        }
    }
}

fn main() {
    let state = GameState::new_with_default_setup();

    println!("{state}");
    // state.do_move([0, 6]);
    // println!("{state}");
    // state.do_move([5, 0]);
    // println!("{state}");
    // state.do_move([99, 0]);
    // println!("{state}");
    // state.do_move([5, 99]);
    // println!("{state}");

    let mut moves = Vec::new();

    for die in 1..=6 {
        // state.get_possible_moves(false, die, &mut moves);
        state.get_possible_moves_double(
            false,
            [1, die],
            &mut moves,
            &mut Vec::new(),
            &mut Vec::new(),
        );
        println!("{moves:2?}");
    }
}
