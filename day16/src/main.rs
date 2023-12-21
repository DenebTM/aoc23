mod beam;
mod grid;
mod pos;
mod tile;

use std::io;

use beam::Beam;
use grid::Grid;

use crate::grid::GridWithBeams;

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
    let mut grid = Grid::from(input);

    let mut beams: Vec<Beam> = vec![Beam::start()];

    // relevant for printing
    let mut num_active_beams = beams.len();
    let mut do_print = false;
    let stdin = io::stdin();

    while beams.len() > 0 {
        let beam = beams.remove(0);

        // only print after advancing each currently active beam one step
        if num_active_beams == 0 {
            do_print = true;
            num_active_beams = beams.len();
        } else {
            num_active_beams -= 1;
        }

        if let Some(tile) = grid.at(beam.pos) {
            let new_beams = tile.beam_result(&beam);
            beams.extend(new_beams);
        }

        if do_print {
            let grid_with_beams = GridWithBeams(&grid, &beams);
            print!("\n{grid_with_beams}");
            let _ = stdin.read_line(&mut String::new()).unwrap();
            do_print = false;
        }
    }

    let energized_count = grid.energized_count();

    println!("part1: {}", energized_count);
}
