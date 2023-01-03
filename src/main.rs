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
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔═════════════════╦═════════════════╗")?;
        for i in 0..12 {
            write!(f, "║")?;
            match self.tiles[i] {
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
                    write!(f, "{colored}{dashes}")?;
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

        writeln!(f, "╚═════════════════╩═════════════════╝")
    }
}

fn main() {
    let state = GameState::new();

    println!("{state}");
}
