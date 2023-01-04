use std::fmt::Display;

use crate::{GameState, Tile::*};

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const COLS: [&str; 2] = ["\x1b[33m", "\x1b[36m"];

        writeln!(f, "  ╔═════════════════╦═════════════════╗")?;
        write!(f, "  ║{}HOME LIGHT\x1b[0m       ║", COLS[0])?;
        writeln!(f, "        {}HOME DARK\x1b[0m║", COLS[1])?;

        {
            let circles = "●".repeat(self.finished[0] as usize);
            let spaces = " ".repeat(15 - self.finished[0] as usize);
            write!(f, "  ║{}{circles}\x1b[0m{spaces}", COLS[0])?;

            write!(f, "  ║  ")?;

            let circles = "●".repeat(self.finished[1] as usize);
            let spaces = " ".repeat(15 - self.finished[1] as usize);
            writeln!(f, "{spaces}{}{circles}\x1b[0m║", COLS[1])?;
        }

        writeln!(f, "  ╠═════════════════╬═════════════════╣")?;
        for i in 0..12 {
            write!(f, "{i:2}║")?;
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

            writeln!(f, "║{}", 23 - i)?;
            if i == 5 {
                writeln!(f, "  ╠═════════════════╬═════════════════╣")?;
            }
        }

        writeln!(f, "  ╠═════════════════╬═════════════════╣")?;
        write!(f, "  ║{}CAPTURED\x1b[0m         ║", COLS[0])?;
        writeln!(f, "         {}CAPTURED\x1b[0m║", COLS[1])?;

        {
            let circles = "●".repeat(self.captured[0] as usize);
            let spaces = " ".repeat(15 - self.captured[0] as usize);
            write!(f, "  ║{}{circles}\x1b[0m{spaces}", COLS[0])?;

            write!(f, "  ║  ")?;

            let circles = "●".repeat(self.captured[1] as usize);
            let spaces = " ".repeat(15 - self.captured[1] as usize);
            writeln!(f, "{spaces}{}{circles}\x1b[0m║", COLS[1])?;
        }

        write!(f, "  ╚═════════════════╩═════════════════╝")
    }
}
