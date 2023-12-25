use std::{collections::HashMap, io};

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

fn find_horiz_reflections(grid: &[String]) -> HashMap<usize, usize> {
    let width = grid[0].len();

    (1..width)
        .map(|x| {
            let mut diff = 0;
            for line in grid {
                let (left, right) = line.split_at(x);
                for (ch1, ch2) in left.chars().rev().zip(right.chars()) {
                    if ch1 != ch2 {
                        diff += 1;
                    }
                }
            }

            (diff, x)
        })
        .collect()
}

fn find_vert_reflections(grid: &[String]) -> HashMap<usize, usize> {
    let height = grid.len();

    (1..height)
        .map(|y| {
            let mut diff = 0;

            let (top, bottom) = grid.split_at(y);
            for (l1, l2) in top.iter().rev().zip(bottom.iter()) {
                for (ch1, ch2) in l1.chars().zip(l2.chars()) {
                    if ch1 != ch2 {
                        diff += 1;
                    }
                }
            }

            (diff, y)
        })
        .collect()
}

fn main() {
    let input = stdio_lines_trimmed();
    let groups: Vec<&[String]> = input.split(|line| line.len() == 0).collect();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for (n, group) in groups.iter().enumerate() {
        let reflect_h = find_horiz_reflections(group);
        let reflect_v = find_vert_reflections(group);

        let get_reflection = |diff: &usize| {
            reflect_h
                .get(diff)
                .map(|h| *h)
                .or(reflect_v.get(diff).map(|v| v * 100))
                .expect(&format!(
                    "Found no reflection with difference {diff} in group {}/{}",
                    n + 1,
                    groups.len(),
                ))
        };

        let part1_part = get_reflection(&0);
        let part2_part = get_reflection(&1);

        part1_sum += part1_part;
        part2_sum += part2_part;
    }

    println!("part1: {part1_sum}");
    println!("part2: {part2_sum}");
}
