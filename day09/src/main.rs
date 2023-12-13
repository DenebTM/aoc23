use std::{io, str::FromStr};

fn get_diffs(nums: &[i32]) -> Vec<i32> {
    nums.iter().zip(&nums[1..]).map(|(a, b)| b - a).collect()
}

fn get_next(nums: &[i32]) -> i32 {
    let diffs = get_diffs(nums);

    let &last_val = nums.last().unwrap();
    let last_diff = if !diffs.iter().all(|&num| num == 0) {
        get_next(&diffs)
    } else {
        0
    };

    last_val + last_diff
}

fn get_prev(nums: &[i32]) -> i32 {
    let diffs = get_diffs(nums);

    let &first_val = nums.first().unwrap();
    let first_diff = if !diffs.iter().all(|&num| num == 0) {
        get_prev(&diffs)
    } else {
        0
    };

    first_val - first_diff
}

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;

    let stdin = io::stdin();
    let mut buf = String::new();
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        let nums: Vec<i32> = buf
            .trim()
            .split_whitespace()
            .flat_map(FromStr::from_str)
            .collect();

        sum1 += get_next(&nums);
        sum2 += get_prev(&nums);

        buf.clear();
    }

    println!("part1: {}", sum1);
    println!("part2: {}", sum2);
}
