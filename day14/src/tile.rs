use std::fmt::Display;

use crate::pos::Pos;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Empty,     // .
    CubeRock,  // #
    RoundRock, // O
}

impl From<char> for TileKind {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Self::CubeRock,
            'O' => Self::RoundRock,
            _ => Self::Empty,
        }
    }
}
impl From<&TileKind> for char {
    fn from(value: &TileKind) -> Self {
        match value {
            TileKind::CubeRock => '#',
            TileKind::RoundRock => 'O',
            _ => '.',
        }
    }
}

impl Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub pos: Pos,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Tile {
    pub fn new(ch: char, pos: impl Into<Pos>) -> Self {
        Self {
            kind: TileKind::from(ch),
            pos: pos.into(),
        }
    }
}
