mod space;
mod tests;

use std::{collections::HashSet, io};

use crate::space::{Galaxy, Space};

// fn stdio_each<T>(func: impl Fn(&str, usize) -> T) -> Vec<T> {
//     let mut output = Vec::new();

//     let stdin = io::stdin();
//     let mut buf = String::new();
//     let mut line_num = 0;
//     while let Ok(count) = stdin.read_line(&mut buf) {
//         if count == 0 {
//             break;
//         }

//         output.push(func(&buf, line_num));

//         line_num += 1;
//         buf.clear();
//     }

//     output
// }

fn stdio_lines_trimmed() -> Vec<String> {
    let stdin = io::stdin();
    stdin
        .lines()
        .flat_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .collect()
}

fn main() {
    let lines = stdio_lines_trimmed();

    let mut space = Space::from(lines, 2);
    let clonedspace = space.clone();
    let gal_pairs: HashSet<(&Galaxy, &Galaxy)> = clonedspace
        .get_gal_pairs()
        .map(|(gal1, gal2)| Some((space.galaxies.get(&gal1)?, space.galaxies.get(&gal2)?)))
        .flatten()
        .collect();

    space.empty_factor = 2;
    let total_dist: usize = gal_pairs
        .iter()
        .map(|(gal1, gal2)| space.get_expanded_dist(gal1, gal2))
        .sum();
    println!("part1: {}", total_dist);

    // no this definitely didn't require me to rebuild the entire thing from scratch why do you ask
    space.empty_factor = 1000000;
    let total_dist: usize = gal_pairs
        .iter()
        .map(|(gal1, gal2)| space.get_expanded_dist(gal1, gal2))
        .sum();
    println!("part2: {}", total_dist);
}
