mod grid;
mod pos;
mod tile;

use std::{collections::HashMap, io};

use crate::{
    grid::Grid,
    pos::{pos, Pos},
    tile::Tile,
};

fn main() {
    let mut grid = Grid::new();

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut y = 0;
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        let row: Vec<Tile> = buf
            .trim()
            .chars()
            .enumerate()
            .map(|(x, ch)| Tile::new(ch, pos(x, y)))
            .collect();

        grid.tiles.push(row);

        y += 1;
        buf.clear();
    }

    /* tile is present in this map -> tile is connected to main loop */
    let mut tile_dists: HashMap<Pos, usize> = HashMap::new();

    let mut work_list = vec![(0, grid.get_start().unwrap())];
    while work_list.len() > 0 {
        let (dist, tile) = work_list.remove(0);

        tile_dists.insert(tile.pos, dist);

        let next_tiles = tile
            .dirs
            .iter()
            .map(|&dir| grid.get_dir(tile.pos, dir))
            .flatten()
            .filter(|next_tile| !tile_dists.contains_key(&next_tile.pos))
            .map(|tile| (dist + 1, tile));

        work_list.extend(next_tiles);
    }

    let max = tile_dists.iter().map(|(_, dist)| dist).max().unwrap();
    println!("part1: {}", max);

    /* part 2: deneb rolls worst line cross counting code ever; asked to leave advent of code */
    let unconnected_tiles: Vec<&Tile> = grid
        .tiles
        .iter()
        .flatten()
        .filter(|&tile| !tile_dists.contains_key(&tile.pos))
        .collect();

    let mut inside = 0;
    for tile in unconnected_tiles {
        let (dot_x, dot_y) = tile.pos.xy();

        let mut lines_crossed = 0;
        let mut last_line_cross: Option<char> = None;
        for ray_x in 0..dot_x {
            let ray_pos = pos(ray_x, dot_y);
            if tile_dists.contains_key(&ray_pos) {
                if let Some(Tile { ch, .. }) = grid.tile_at(ray_pos).filter(|t| t.ch != '-') {
                    lines_crossed += match last_line_cross {
                        None => 1,
                        Some(last_cross) => match (last_cross, ch) {
                            // e.g. L7 or F---J represents one continuous vertical line that should not be double-counted
                            ('L', '7') | ('F', 'J') => 0,
                            _ => 1,
                        },
                    };

                    last_line_cross = Some(ch);
                }
            }
        }

        if lines_crossed % 2 == 1 {
            inside += 1;
        }
    }
    println!("part2: {}", inside);
}
