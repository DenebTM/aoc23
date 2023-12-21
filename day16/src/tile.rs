use std::{collections::HashSet, fmt::Display};

use crate::beam::{Beam, BeamDir};

pub enum TileKind {
    Empty,      // .
    MirrorNWSE, // /
    MirrorNESW, // \
    SplitNS,    // -
    SplitEW,    // |
}

impl From<char> for TileKind {
    fn from(ch: char) -> Self {
        match ch {
            '/' => Self::MirrorNWSE,
            '\\' => Self::MirrorNESW,
            '-' => Self::SplitNS,
            '|' => Self::SplitEW,
            _ => Self::Empty,
        }
    }
}
impl From<&TileKind> for char {
    fn from(value: &TileKind) -> Self {
        match value {
            TileKind::MirrorNWSE => '/',
            TileKind::MirrorNESW => '\\',
            TileKind::SplitNS => '-',
            TileKind::SplitEW => '|',
            _ => '.',
        }
    }
}

impl Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

pub struct Tile {
    kind: TileKind,
    energized_dirs: HashSet<BeamDir>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.energized_dirs.len() > 0 {
            write!(f, "#")
        } else {
            self.kind.fmt(f)
        }
    }
}

impl Tile {
    pub fn new(ch: char) -> Self {
        Self {
            kind: TileKind::from(ch),
            energized_dirs: HashSet::new(),
        }
    }

    pub fn is_energized(&self) -> bool {
        self.energized_dirs.len() > 0
    }

    pub fn beam_result(&mut self, beam: &Beam) -> Vec<Beam> {
        // memorize incoming beam to avoid duplicate work
        if self.energized_dirs.contains(&beam.dir) {
            return vec![];
        }
        self.energized_dirs.insert(beam.dir);

        match self.kind {
            TileKind::Empty => vec![beam.forward()],
            TileKind::MirrorNWSE => match beam.dir {
                BeamDir::Left | BeamDir::Right => vec![beam.left()],
                BeamDir::Down | BeamDir::Up => vec![beam.right()],
            },
            TileKind::MirrorNESW => match beam.dir {
                BeamDir::Left | BeamDir::Right => vec![beam.right()],
                BeamDir::Down | BeamDir::Up => vec![beam.left()],
            },

            TileKind::SplitNS => match beam.dir {
                BeamDir::Left | BeamDir::Right => vec![beam.forward()],
                BeamDir::Up | BeamDir::Down => {
                    vec![beam.left(), beam.right()]
                }
            },
            TileKind::SplitEW => match beam.dir {
                BeamDir::Up | BeamDir::Down => vec![beam.forward()],
                BeamDir::Left | BeamDir::Right => {
                    vec![beam.left(), beam.right()]
                }
            },
        }
    }
}
