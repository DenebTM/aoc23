use std::{collections::HashMap, fmt::Display};

use crate::{pos::Pos, tile::TileKind};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    tiles: HashMap<Pos, TileKind>,
}

impl From<Vec<String>> for Grid {
    fn from(mat: Vec<String>) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = HashMap::new();

        for (y, line) in mat.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    tiles.insert((x, y).into(), TileKind::from(ch));
                }
                if x > width {
                    width = x + 1;
                }
                if y > height {
                    height = y + 1;
                }
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}

impl Grid {
    pub fn at(&self, pos: Pos) -> Option<TileKind> {
        if pos.0 >= 0
            && pos.1 >= 0
            && pos.0 < (self.width as isize)
            && pos.1 < (self.height as isize)
        {
            self.tiles.get(&pos).copied().or(Some(TileKind::Empty))
        } else {
            None
        }
    }

    pub fn move_rock(self, pos: Pos, new_pos: Pos) -> Self {
        let Self {
            width,
            height,
            mut tiles,
        } = self;
        if pos != new_pos {
            tiles.remove(&pos);
            tiles.insert(new_pos, TileKind::RoundRock);
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn get_round_rocks(&self) -> Vec<Pos> {
        self.tiles
            .iter()
            .filter_map(|(&pos, &kind)| {
                if kind == TileKind::RoundRock {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.at((x, y).into()) {
                    Some(tile) => {
                        write!(f, "{tile}")?;
                    }
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
