mod beam;
mod grid;
mod pos;
mod tile;

use std::io;

use beam::{Beam, BeamDir};
use grid::Grid;

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

fn get_energized_tile_count(init_grid: &Grid, init_beam: Beam) -> usize {
    let mut grid = init_grid.clone();
    let mut beams: Vec<Beam> = vec![init_beam];

    while beams.len() > 0 {
        let beam = beams.remove(0);

        if let Some(tile) = grid.at(beam.pos) {
            let new_beams = tile.beam_result(&beam);
            beams.extend(new_beams);
        }
    }

    grid.energized_count()
}

fn main() {
    let input = stdio_lines_trimmed();
    let final_row = input.len() - 1;
    let final_col = input[0].len() - 1;
    let grid = Grid::from(input);

    let part1_beam = Beam::new((0, 0), BeamDir::Right);
    let energized_count = get_energized_tile_count(&grid, part1_beam);
    println!("part1: {}", energized_count);

    let part2_beams = [
        (0..final_row)
            .map(|row| {
                [
                    Beam::new((0, row), BeamDir::Right),
                    Beam::new((final_col, row), BeamDir::Left),
                ]
            })
            .collect::<Vec<_>>(),
        (0..final_col)
            .map(|col| {
                [
                    Beam::new((col, 0), BeamDir::Down),
                    Beam::new((col, final_row), BeamDir::Up),
                ]
            })
            .collect(),
    ]
    .concat()
    .concat();

    let max_energized_count = part2_beams
        .iter()
        .map(|&beam| get_energized_tile_count(&grid, beam))
        .max()
        .unwrap_or(0);
    println!("part2: {}", max_energized_count);
}
