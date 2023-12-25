mod grid;
mod pos;
mod tile;

use std::{collections::HashMap, io};

use crate::{grid::Grid, pos::Dir, tile::TileKind};

#[allow(dead_code)]
fn stdio_each<T>(func: impl Fn(&str, usize) -> T) -> Vec<T> {
    let mut output = Vec::new();

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut line_num = 0;
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        output.push(func(&buf, line_num));

        line_num += 1;
        buf.clear();
    }

    output
}

#[allow(dead_code)]
fn stdio_lines_trimmed() -> Vec<String> {
    let stdin = io::stdin();
    stdin
        .lines()
        .flat_map(move |line| line.ok())
        .map(move |line| line.trim().to_string())
        .collect()
}

fn main() {
    let input = stdio_lines_trimmed();
    let grid = Grid::from(input);

    // move all round rocks as far north as possible
    let mut grid_north = grid.clone();
    for tile in grid_north.get_round_rocks() {
        while grid.at(tile.pos - (0, 1)).map(|tile| tile.kind) == Some(TileKind::Empty) {
            tile.pos -= (0, 1);
        }
    }

    let mut part1_sum = 0;

    println!("part1: {part1_sum}");
}
