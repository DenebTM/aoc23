mod grid;
mod pos;
mod tile;

use std::{io, thread::sleep, time::Duration};

use pos::{Dir, Pos};
use tile::Tile;

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

fn tilt(grid: Grid, dir: Dir) -> Grid {
    let mut grid = grid;

    let mut round_rocks = grid.get_round_rocks();
    round_rocks.sort();

    for tile in round_rocks {
        let mut new_pos = tile.pos;
        while grid.at(new_pos + dir).map(|tile| tile.kind) == Some(TileKind::Empty) {
            new_pos += dir;
        }

        grid = grid.move_tile(tile.pos, new_pos);
    }

    grid
}

fn north_load(grid: &Grid) -> usize {
    grid.get_round_rocks()
        .iter()
        .map(|Tile { pos: Pos(_, y), .. }| grid.height - (*y as usize))
        .sum()
}

fn spin_cycle(grid: Grid) -> Grid {
    tilt(
        tilt(tilt(tilt(grid, Dir::NORTH), Dir::WEST), Dir::SOUTH),
        Dir::EAST,
    )
}

fn main() {
    let input = stdio_lines_trimmed();
    let grid = Grid::from(input);

    let grid_north = tilt(grid.clone(), Dir::NORTH);
    let part1_sum = north_load(&grid_north);
    println!("\n{grid_north}");
    println!("part1: {part1_sum}");

    sleep(Duration::from_millis(1000));

    let mut grid_spun = grid;
    for cycle in 1..=1_000_000_000 {
        println!("Cycle {cycle}/1,000,000,000");

        let new_grid = spin_cycle(grid_spun.clone());
        if grid_spun == new_grid {
            break;
        } else {
            grid_spun = new_grid;
        }
    }
    let part2_sum = north_load(&grid_spun);

    println!("\n{grid_spun}");
    println!("part2: {part2_sum}");
}
