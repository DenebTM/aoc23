use std::fmt::Display;

use colored::{Color, Colorize};

use crate::{
    beam::{Beam, BeamDir},
    pos::Pos,
    tile::Tile,
};

#[derive(Clone)]
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

pub struct GridWithBeams<'a>(pub &'a Grid, pub &'a Vec<Beam>);
impl<'a> Display for GridWithBeams<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GridWithBeams(grid, beams) = self;

        let beam_color = |pos: Pos| {
            if beams.iter().filter(|beam| beam.pos == pos).count() > 1 {
                return Some(Color::BrightWhite);
            }

            for beam in beams.iter() {
                if beam.pos == pos {
                    return Some(match beam.dir {
                        BeamDir::Up => Color::BrightRed,
                        BeamDir::Right => Color::BrightGreen,
                        BeamDir::Down => Color::BrightBlue,
                        BeamDir::Left => Color::BrightMagenta,
                    });
                }
            }

            None
        };

        for (y, line) in grid.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                write!(
                    f,
                    "{}",
                    match beam_color((x, y).into()) {
                        Some(color) => tile.to_string().color(color),
                        None => tile.to_string().into(),
                    }
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
