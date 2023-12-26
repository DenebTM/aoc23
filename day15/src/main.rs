mod hash;

use std::io;

use crate::hash::hash;

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
    let input = stdio_lines_trimmed().concat();

    let part1_sum = input
        .split(',')
        .fold(0, |acc, string| acc + (hash(string) as usize));

    println!("part1: {}", part1_sum);

    let mut boxes: Vec<Vec<(&str, u8)>> = (0..256).map(|_| Vec::new()).collect();
    for string in input.split(',') {
        if string.contains('=') {
            let (key, val) = string.split_at(string.find('=').unwrap());
            let hash = hash(key);
            let new_val = val[1..].parse::<u8>().unwrap();

            let b = &mut boxes[hash];
            'replace_or_push: {
                for (existing_key, existing_val) in b.iter_mut() {
                    if &key == existing_key {
                        *existing_val = new_val;
                        break 'replace_or_push;
                    }
                }
                b.push((key, new_val));
            }
        } else {
            let (key, _) = string.split_at(string.find('-').unwrap());
            let hash = hash(key);

            let b = &mut boxes[hash];
            b.retain(|(existing_key, _)| existing_key != &key);
        }
    }

    let mut part2_sum = 0;
    for box_num in 0..256 {
        for (lens_slot, (_, val)) in boxes[box_num].iter().enumerate() {
            part2_sum += (1 + box_num) * (1 + lens_slot) * (*val as usize);
        }
    }

    println!("part2: {}", part2_sum);
}
