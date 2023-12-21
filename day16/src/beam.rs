use crate::pos::Pos;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BeamDir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub struct Beam {
    pub pos: Pos,
    pub dir: BeamDir,
}

impl Beam {
    pub fn start() -> Self {
        Self {
            pos: (0, 0).into(),
            dir: BeamDir::Right,
        }
    }

    pub fn forward(self) -> Self {
        Self {
            pos: self.pos
                + match self.dir {
                    BeamDir::Up => (0, -1),
                    BeamDir::Right => (1, 0),
                    BeamDir::Down => (0, 1),
                    BeamDir::Left => (-1, 0),
                },
            dir: self.dir,
        }
    }

    pub fn right(self) -> Self {
        Self {
            dir: match self.dir {
                BeamDir::Up => BeamDir::Right,
                BeamDir::Right => BeamDir::Down,
                BeamDir::Down => BeamDir::Left,
                BeamDir::Left => BeamDir::Up,
            },
            pos: self.pos,
        }
        .forward()
    }

    pub fn left(self) -> Self {
        Self {
            dir: match self.dir {
                BeamDir::Up => BeamDir::Left,
                BeamDir::Left => BeamDir::Down,
                BeamDir::Down => BeamDir::Right,
                BeamDir::Right => BeamDir::Up,
            },
            pos: self.pos,
        }
        .forward()
    }
}
