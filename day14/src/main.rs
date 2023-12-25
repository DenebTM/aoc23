mod grid;
mod pos;
mod tile;

use std::io;

use crate::{grid::Grid, tile::TileKind};

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

    let mut grid_north = grid.clone();
    let mut part1_sum = 0;

    let mut round_rocks = grid_north.get_round_rocks();
    round_rocks.sort();

    // move all round rocks as far north as possible
    for tile in round_rocks {
        let mut new_pos = tile.pos;
        while grid_north.at(new_pos - (0, 1)).map(|tile| tile.kind) == Some(TileKind::Empty) {
            new_pos -= (0, 1);
        }

        grid_north = grid_north.move_tile(tile.pos, new_pos);
        part1_sum += grid.height - (new_pos.1 as usize);
    }

    println!("\n{grid_north}");

    println!("part1: {part1_sum}");
}
