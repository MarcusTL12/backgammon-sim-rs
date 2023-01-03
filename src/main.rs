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
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..12 {
            match self.tiles[i] {
                Empty => (),
                Light(n) => (),
                Dark(n) => (),
            }
        }

        Ok(())
    }
}

fn main() {
    let state = GameState::new();

    println!("{state:?}");
}
