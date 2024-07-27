mod game_display;
mod evaluator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Light(u8),
    Dark(u8),
}

use std::{collections::HashMap, mem::transmute};

use arrayvec::ArrayVec;
use Tile::*;

const SPECIAL_MOVE: u8 = 99;

type StateMap = HashMap<GameState, ArrayVec<[u8; 2], 4>>;

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

fn count_trues<const N: usize>(bits: &[bool; N]) -> usize {
    bits.iter().filter(|&&x| x).count()
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

    pub fn get_possible_moves_double_ordered(
        &self,
        turn: bool,
        dice: [u8; 2],
        move_buf: &mut Vec<Vec<[u8; 2]>>,
        seen: &mut [HashMap<GameState, ArrayVec<[u8; 2], 4>>; 5],
    ) {
        // u for unused
        // w for wasteful
        let [seen_ord, seen_1w, seen_2w, seen_1u, seen_1u1w] = seen;

        let mut buf1 = move_buf.pop().unwrap_or_default();
        let mut buf2 = move_buf.pop().unwrap_or_default();

        let wasteful1 = self.get_possible_moves(turn, dice[0], &mut buf1);

        for &m1 in &buf1 {
            let mut state = self.clone();
            state.do_move(m1);

            let wasteful2 = self.get_possible_moves(turn, dice[1], &mut buf2);

            if buf2.is_empty() {
                if wasteful1 {
                    &mut *seen_1u1w
                } else {
                    &mut *seen_1u
                }
                .insert(state, [m1].into_iter().collect());
            } else {
                for &m2 in &buf2 {
                    let mut state = state.clone();
                    state.do_move(m2);

                    match count_trues(&[wasteful1, wasteful2]) {
                        0 => &mut *seen_ord,
                        1 => &mut *seen_1w,
                        2 => &mut *seen_2w,
                        _ => unreachable!(),
                    }
                    .insert(state, [m1, m2].into_iter().collect());
                }
            }
        }

        move_buf.push(buf2);
        move_buf.push(buf1);
    }

    fn get_possible_moves_quadruple_ordered(
        &self,
        turn: bool,
        die: u8,
        move_buf: &mut Vec<Vec<[u8; 2]>>,
        seen: &mut [HashMap<GameState, ArrayVec<[u8; 2], 4>>; 14],
    ) {
        // u for unused
        // w for wasteful
        // transmuting because rustfmt is weird.
        let tmp: &mut (
            [StateMap; 5],
            [StateMap; 4],
            [StateMap; 3],
            [StateMap; 2],
        ) = unsafe { transmute(seen) };
        let (
            [seen_ord, seen_1w, seen_2w, seen_3w, seen_4w],
            [seen_1u, seen_1u1w, seen_1u2w, seen_1u3w],
            [seen_2u, seen_2u1w, seen_2u2w],
            [seen_3u, seen_3u1w],
        ) = tmp;

        let mut buf1 = move_buf.pop().unwrap_or_default();
        let mut buf2 = move_buf.pop().unwrap_or_default();
        let mut buf3 = move_buf.pop().unwrap_or_default();
        let mut buf4 = move_buf.pop().unwrap_or_default();

        let wasteful1 = self.get_possible_moves(turn, die, &mut buf1);

        for &m1 in &buf1 {
            let mut state = self.clone();
            state.do_move(m1);

            let wasteful2 = state.get_possible_moves(turn, die, &mut buf2);

            if buf2.is_empty() {
                if wasteful1 {
                    &mut *seen_3u1w
                } else {
                    &mut *seen_3u
                }
                .insert(state, [m1].into_iter().collect());
            } else {
                for &m2 in &buf2 {
                    let mut state = state.clone();
                    state.do_move(m2);

                    let wasteful3 =
                        state.get_possible_moves(turn, die, &mut buf3);

                    if buf3.is_empty() {
                        match count_trues(&[wasteful1, wasteful2]) {
                            0 => &mut *seen_2u,
                            1 => &mut *seen_2u1w,
                            2 => &mut *seen_2u2w,
                            _ => unreachable!(),
                        }
                        .insert(state, [m1, m2].into_iter().collect());
                    } else {
                        for &m3 in &buf3 {
                            let mut state = state.clone();
                            state.do_move(m3);

                            let wasteful4 =
                                state.get_possible_moves(turn, die, &mut buf4);

                            if buf4.is_empty() {
                                match count_trues(&[
                                    wasteful1, wasteful2, wasteful3,
                                ]) {
                                    0 => &mut *seen_1u,
                                    1 => &mut *seen_1u1w,
                                    2 => &mut *seen_1u2w,
                                    3 => &mut *seen_1u3w,
                                    _ => unreachable!(),
                                }
                                .insert(
                                    state,
                                    [m1, m2, m3].into_iter().collect(),
                                );
                            } else {
                                for &m4 in &buf4 {
                                    let mut state = state.clone();
                                    state.do_move(m4);

                                    match count_trues(&[
                                        wasteful1, wasteful2, wasteful3,
                                        wasteful4,
                                    ]) {
                                        0 => &mut *seen_ord,
                                        1 => &mut *seen_1w,
                                        2 => &mut *seen_2w,
                                        3 => &mut *seen_3w,
                                        4 => &mut *seen_4w,
                                        _ => unreachable!(),
                                    }
                                    .insert(
                                        state,
                                        [m1, m2, m3, m4].into_iter().collect(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        move_buf.push(buf4);
        move_buf.push(buf3);
        move_buf.push(buf2);
        move_buf.push(buf1);
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
            let mut seen =
                [(); 5].map(|()| seen_states_buf.pop().unwrap_or_default());

            for seen in &mut seen {
                seen.clear();
            }

            self.get_possible_moves_double_ordered(
                turn, dice, move_buf, &mut seen,
            );

            self.get_possible_moves_double_ordered(
                turn,
                [dice[1], dice[0]],
                move_buf,
                &mut seen,
            );

            if let Some(seen) = seen.iter_mut().find(|seen| !seen.is_empty()) {
                moves.extend(seen.drain().map(|(_, x)| x));
            }

            for seen in seen {
                seen_states_buf.push(seen);
            }

            moves.sort();
        } else {
            let mut seen =
                [(); 14].map(|()| seen_states_buf.pop().unwrap_or_default());

            for seen in &mut seen {
                seen.clear();
            }

            self.get_possible_moves_quadruple_ordered(
                turn, dice[0], move_buf, &mut seen,
            );

            if let Some(seen) = seen.iter_mut().find(|seen| !seen.is_empty()) {
                moves.extend(seen.drain().map(|(_, x)| x));
            }

            for seen in seen {
                seen_states_buf.push(seen);
            }
        }
    }
}
