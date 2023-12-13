use crate::{
    pos::{Dir, Pos},
    tile::Tile,
};

#[derive(Clone)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    pub fn tile_at(&self, pos: Pos) -> Option<Tile> {
        let (x, y) = pos.xy();
        self.tiles
            .get(y)
            .map(|row| {
                row.get(x).map(|tile| {
                    if tile.ch == 'S' {
                        self.get_start()
                    } else {
                        tile.not_ground()
                    }
                })
            })
            .flatten()
            .flatten()
    }

    pub fn get_start(&self) -> Option<Tile> {
        for Tile { ch, pos, .. } in self.tiles.iter().flatten().cloned() {
            if ch == 'S' {
                let mut dirs: Vec<Dir> = Vec::new();
                for dir in [Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST] {
                    self.tile_at(pos + dir).map(|adj_tile| {
                        if adj_tile.dirs.contains(&-dir) {
                            dirs.push(dir)
                        }
                    });
                }

                return Some(Tile { ch, pos, dirs });
            }
        }

        None
    }

    pub fn get_dir(&self, pos: Pos, dir: Dir) -> Option<Tile> {
        self.tile_at(pos)
            .filter(|tile| tile.dirs.contains(&dir))
            .and(self.tile_at(pos + dir))
    }
}
