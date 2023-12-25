use std::io;

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

fn find_horiz_reflection(grid: &[String]) -> Option<usize> {
    let width = grid[0].len();

    'outer: for x in 1..width {
        for line in grid {
            let (left, right) = line.split_at(x);
            for (ch1, ch2) in left.chars().rev().zip(right.chars()) {
                if ch1 != ch2 {
                    continue 'outer;
                }
            }
        }

        return Some(x);
    }

    None
}

fn find_vert_reflection(grid: &[String]) -> Option<usize> {
    let height = grid.len();

    'outer: for y in 1..height {
        let (top, bottom) = grid.split_at(y);
        for (l1, l2) in top.iter().rev().zip(bottom.iter()) {
            if l1 != l2 {
                continue 'outer;
            }
        }

        return Some(y);
    }

    None
}

fn main() {
    let input = stdio_lines_trimmed();
    let groups: Vec<&[String]> = input.split(|line| line.len() == 0).collect();

    let mut sum = 0;

    for (n, group) in groups.iter().enumerate() {
        let reflect_h = find_horiz_reflection(group);
        let reflect_v = find_vert_reflection(group);

        let part = reflect_h.or(reflect_v.map(|v| v * 100)).expect(&format!(
            "Found no reflection in group {}/{}",
            n + 1,
            groups.len(),
        ));
        sum += part;
    }

    println!("part1: {sum}");
}
