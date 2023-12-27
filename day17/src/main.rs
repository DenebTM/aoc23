mod a_star;
mod grid;
mod pos;

use std::io;

use a_star::a_star;
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

fn main() {
    let input = stdio_lines_trimmed();
    let grid = Grid::from(input);

    let part1_path = a_star(
        &grid,
        (0, 0).into(),
        (grid.width() - 1, grid.height() - 1).into(),
        0,
        3,
    );
    println!("part1: {:?}", part1_path.map(|p| p.total_cost));

    let part2_path = a_star(
        &grid,
        (0, 0).into(),
        (grid.width() - 1, grid.height() - 1).into(),
        4,
        10,
    );
    println!("part2: {:?}", part2_path.map(|p| p.total_cost));
}
