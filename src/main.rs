mod game_display;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Light(u8),
    Dark(u8),
}

use Tile::*;
use arrayvec::ArrayVec;

const SPECIAL_MOVE: u8 = 99;

#[derive(Debug)]
pub struct GameState {
    tiles: [Tile; 24],
    captured: [u8; 2],
    finished: [u8; 2],
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

    pub fn get_possible_moves(
        &self,
        turn: bool, // false => Light, true => Dark
        die: u8,
        wasteful: bool,
        moves: &mut Vec<[u8; 2]>,
    ) {
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
                    | (Some(Dark(_)), true) = (
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

                if wasteful && moves.is_empty() {
                    let mut i = die_pos as isize;
                    while let Some(t) = self.tiles.get(i as usize) {
                        if let (Light(_), false) | (Dark(_), true) = (t, turn) {
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
    }

    pub fn get_possible_moves_double(
        &self,
        turn: bool,
        dice: [u8; 2],
        moves: &mut Vec<ArrayVec<[u8; 2], 4>>,
    ) {
    }
}

fn main() {
    // let state = GameState::new_with_default_setup();
    let mut state = GameState::new();
    state.tiles[0] = Dark(15);
    state.tiles[23] = Light(15);

    println!("{state}");

    let mut moves = Vec::new();

    for die in 1..=6 {
        state.get_possible_moves(false, die, true, &mut moves);
        println!("{moves:2?}");
    }
}
