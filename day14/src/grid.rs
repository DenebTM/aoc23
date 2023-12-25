use std::{collections::HashMap, fmt::Display};

use crate::{
    pos::Pos,
    tile::{Tile, TileKind},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    tiles: HashMap<Pos, Tile>,
}

impl From<Vec<String>> for Grid {
    fn from(mat: Vec<String>) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = HashMap::new();

        for (y, line) in mat.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    tiles.insert((x, y).into(), Tile::new(ch, (x, y)));
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
    pub fn at(&self, pos: Pos) -> Option<Tile> {
        if pos.0 >= 0
            && pos.1 >= 0
            && pos.0 < (self.width as isize)
            && pos.1 < (self.height as isize)
        {
            self.tiles.get(&pos).copied().or(Some(Tile {
                kind: TileKind::Empty,
                pos,
            }))
        } else {
            None
        }
    }
    pub fn move_tile(self, pos: Pos, new_pos: Pos) -> Self {
        let Self {
            width,
            height,
            mut tiles,
        } = self;
        tiles.get(&pos).copied().map(|tile| {
            tiles.remove(&pos);
            tiles.insert(
                new_pos,
                Tile {
                    kind: tile.kind,
                    pos: new_pos,
                },
            );
        });

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tiles_of_kind(&self, kind: TileKind) -> Vec<Tile> {
        self.tiles
            .values()
            .filter(move |tile| tile.kind == kind)
            .copied()
            .collect()
    }

    pub fn get_round_rocks(&self) -> Vec<Tile> {
        self.get_tiles_of_kind(TileKind::RoundRock)
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
