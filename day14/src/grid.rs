use crate::{
    pos::Pos,
    tile::{Tile, TileKind},
};

#[derive(Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl From<Vec<String>> for Grid {
    fn from(mat: Vec<String>) -> Self {
        let mut tiles = Vec::new();

        for (y, line) in mat.iter().enumerate() {
            let mut tiles_line = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                tiles_line.push(Tile::new(ch, (x, y)));
            }

            tiles.push(tiles_line)
        }

        Self { tiles }
    }
}

impl Grid {
    pub fn at(&mut self, pos: Pos) -> Option<&mut Tile> {
        let (x, y): (usize, usize) = pos.into();
        self.tiles.get_mut(y).map(|line| line.get_mut(x)).flatten()
    }

    pub fn get_tiles_of_kind(&mut self, kind: TileKind) -> impl Iterator<Item = &mut Tile> {
        self.tiles
            .iter_mut()
            .flatten()
            .filter(move |tile| tile.kind == kind)
    }

    pub fn get_round_rocks(&mut self, kind: TileKind) -> impl Iterator<Item = &mut Tile> {
        self.get_tiles_of_kind(TileKind::RoundRock)
    }
}
