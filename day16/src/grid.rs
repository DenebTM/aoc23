use std::fmt::Display;

use crate::{pos::Pos, tile::Tile};

pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl From<Vec<String>> for Grid {
    fn from(mat: Vec<String>) -> Self {
        let mut tiles = Vec::new();

        for line in mat {
            let mut tiles_line = Vec::new();
            for ch in line.chars() {
                tiles_line.push(Tile::new(ch));
            }

            tiles.push(tiles_line)
        }

        Self { tiles }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for line in &self.tiles {
        //     for tile in line {
        //         write!(f, "{tile}")?;
        //     }

        //     write!(f, "\n")?;
        // }

        // Ok(())

        self.tiles.iter().fold(Ok(()), |_, line| {
            line.iter()
                .fold(Ok(()), |_, tile| write!(f, "{tile}"))
                .and(write!(f, "\n"))
        })
    }
}

impl Grid {
    pub fn at(&mut self, pos: Pos) -> Option<&mut Tile> {
        let (x, y): (usize, usize) = pos.into();
        self.tiles.get_mut(y).map(|line| line.get_mut(x)).flatten()
    }

    pub fn energized_count(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|tile| tile.is_energized())
            .count()
    }
}
