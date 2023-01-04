use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Light(u8),
    Dark(u8),
}

use Tile::*;

#[derive(Debug)]
pub struct GameState {
    tiles: [Tile; 24],
    captured: [u8; 2],
    finished: [u8; 2],
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const COLS: [&str; 2] = ["\x1b[33m", "\x1b[36m"];

        writeln!(f, "╔═════════════════╦═════════════════╗")?;
        write!(f, "║{}HOME LIGHT\x1b[0m       ║", COLS[0])?;
        writeln!(f, "        {}HOME DARK\x1b[0m║", COLS[1])?;

        {
            let circles = "●".repeat(self.finished[0] as usize);
            let spaces = " ".repeat(15 - self.finished[0] as usize);
            write!(f, "║{}{circles}\x1b[0m{spaces}", COLS[0])?;

            write!(f, "  ║  ")?;

            let circles = "●".repeat(self.finished[1] as usize);
            let spaces = " ".repeat(15 - self.finished[1] as usize);
            writeln!(f, "{spaces}{}{circles}\x1b[0m║", COLS[1])?;
        }

        writeln!(f, "╠═════════════════╬═════════════════╣")?;
        for i in 0..12 {
            write!(f, "║")?;
            match self.tiles[i] {
                Empty => print!("---------------"),
                t => {
                    let (col, n) = match t {
                        Light(n) => (COLS[0], n),
                        Dark(n) => (COLS[1], n),
                        Empty => unreachable!(),
                    };

                    let circles = "●".repeat(n as usize);
                    let dashes = "-".repeat(15 - n as usize);
                    write!(f, "{col}{circles}\x1b[0m{dashes}")?;
                }
            }

            write!(f, "  ║  ")?;

            match self.tiles[23 - i] {
                Empty => print!("---------------"),
                t => {
                    let (col, n) = match t {
                        Light(n) => ("\x1b[33m", n),
                        Dark(n) => ("\x1b[36m", n),
                        Empty => unreachable!(),
                    };

                    let circles = "●".repeat(n as usize);
                    let colored = format!("{col}{circles}\x1b[0m");
                    let dashes = "-".repeat(15 - n as usize);
                    write!(f, "{dashes}{colored}")?;
                }
            }

            if i == 5 {
                writeln!(f, "║\n╠═════════════════╬═════════════════╣")?;
            } else {
                writeln!(f, "║")?;
            }
        }

        writeln!(f, "╠═════════════════╬═════════════════╣")?;
        write!(f, "║{}CAPTURED\x1b[0m         ║", COLS[0])?;
        writeln!(f, "         {}CAPTURED\x1b[0m║", COLS[1])?;

        {
            let circles = "●".repeat(self.captured[0] as usize);
            let spaces = " ".repeat(15 - self.captured[0] as usize);
            write!(f, "║{}{circles}\x1b[0m{spaces}", COLS[0])?;

            write!(f, "  ║  ")?;

            let circles = "●".repeat(self.captured[1] as usize);
            let spaces = " ".repeat(15 - self.captured[1] as usize);
            writeln!(f, "{spaces}{}{circles}\x1b[0m║", COLS[1])?;
        }

        write!(f, "╚═════════════════╩═════════════════╝")
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            tiles: [
                Light(2),
                Empty,
                Empty,
                Empty,
                Empty,
                Dark(5),
                Empty,
                Dark(3),
                Empty,
                Empty,
                Empty,
                Light(5),
                Dark(5),
                Empty,
                Empty,
                Empty,
                Light(3),
                Empty,
                Light(5),
                Empty,
                Empty,
                Empty,
                Empty,
                Dark(2),
            ],
            captured: [0, 0],
            finished: [0, 0],
        }
    }

    pub fn is_all_home(&self) -> [bool; 2] {
        let mut home = [true; 2];

        for t in &self.tiles[..6] {
            if let Dark(_) = t {
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
            if let Light(_) = t {
                home[0] = false;
            }
        }

        home
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
                    23 - die as usize
                } else {
                    die as usize
                };

                if let (Light(_), false) | (Dark(_), true) =
                    (self.tiles[die_pos], turn)
                {
                    moves.push([die_pos as u8, 255]);
                }

                // At this point if no moves has been pushed,
                // we can check for 'wasteful' moves.

                if wasteful && moves.is_empty() {
                    let mut i = die_pos as isize;
                    while let Some(t) = self.tiles.get(i as usize) {
                        if let (Light(_), false) | (Dark(_), true) = (t, turn) {
                            moves.push([i as u8, 255]);
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
                    moves.push([255, i as u8]);
                }
            }
        }
    }
}

fn main() {
    let state = GameState::new();

    println!("{state}");
}
