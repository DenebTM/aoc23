use crate::pos::{Dir, Pos};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub ch: char,
    pub pos: Pos,

    pub dirs: Vec<Dir>,
}

impl Tile {
    pub fn new(ch: char, pos: Pos) -> Self {
        Self {
            ch,
            pos,
            dirs: match ch {
                '|' => vec![Dir::NORTH, Dir::SOUTH],
                '-' => vec![Dir::EAST, Dir::WEST],
                'L' => vec![Dir::NORTH, Dir::EAST],
                'J' => vec![Dir::NORTH, Dir::WEST],
                '7' => vec![Dir::SOUTH, Dir::WEST],
                'F' => vec![Dir::SOUTH, Dir::EAST],
                'S' => vec![Dir::NORTH, Dir::SOUTH, Dir::EAST, Dir::WEST],
                _ => vec![],
            },
        }
    }

    pub fn not_ground(&self) -> Option<Self> {
        match self.ch {
            '.' => None,
            _ => Some(self.clone()),
        }
    }
}
