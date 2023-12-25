use std::fmt::Display;

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
