use std::{collections::HashSet, io, iter::repeat, str::FromStr};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
        .flat_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .collect()
}

fn placements(line: &str, group_len: usize) -> HashSet<usize> {
    if line.len() < group_len {
        return HashSet::new();
    }

    let mut pos = HashSet::new();

    for start in 0..=(line.len() - group_len) {
        let subline = &line[start..];

        // found contiguous [#?] group
        if subline[..group_len].chars()
            .all(|ch| matches!(ch, '#' | '?'))
            // at the start of the string, or preceded by a . or ?
            && (start == 0 || matches!(line.chars().nth(start - 1), Some('.') | Some('?')))
            // at the end of the string, or succeeded by a . or ?
            && (start + group_len == line.len()
                || matches!(line.chars().nth(start + group_len), Some('.') | Some('?')))
        {
            pos.insert(start);
        }
    }

    pos
}

#[test]
fn test_placements() {
    let test_input = ".###.";
    let positions = placements(test_input, 3);
    assert_eq!(positions, HashSet::from([1]));

    let test_input = "?????";
    let positions = placements(test_input, 3);
    assert_eq!(positions, HashSet::from([0, 1, 2]));

    let test_input = "????.";
    let positions = placements(test_input, 3);
    assert_eq!(positions, HashSet::from([0, 1]));

    let test_input = ".????";
    let positions = placements(test_input, 3);
    assert_eq!(positions, HashSet::from([1, 2]));

    let test_input = "#????";
    let positions = placements(test_input, 3);
    assert_eq!(positions, HashSet::from([0, 2]));

    let test_input = "?????";
    let positions = placements(test_input, 1);
    assert_eq!(positions, HashSet::from([0, 1, 2, 3, 4]));

    let test_input = "#####";
    let positions = placements(test_input, 5);
    assert_eq!(positions, HashSet::from([0]));
}

fn has_all_fixed(placement: &[usize], groups: &[usize], line: &str) -> bool {
    for (ind, ch) in line.chars().enumerate() {
        if ch == '#' {
            let mut hash_found = false;
            for (&pos, &len) in placement.iter().zip(groups.iter()) {
                if ind >= pos && ind < (pos + len) {
                    hash_found = true;
                }
            }
            // if all groups were checked and the # does not correspond to any of them,
            // the placement is invalid
            if !hash_found {
                return false;
            }
        }
    }

    true
}

#[test]
fn test_has_all_fixed() {
    assert!(has_all_fixed(&[0, 4], &[1, 1], "???.#") == true);
    assert!(has_all_fixed(&[1, 4], &[1, 1], "???.#") == true);
    assert!(has_all_fixed(&[2, 4], &[1, 1], "???.#") == true);
    assert!(has_all_fixed(&[0, 2], &[1, 1], "???.#") == false);
}

fn place_groups_unfiltered(line: &str, groups: &[usize]) -> HashSet<Vec<usize>> {
    if let Some(&group_len) = groups.first() {
        let pos = placements(line, group_len);
        if groups.len() == 1 {
            pos.iter().map(|&p| vec![p]).collect()
        } else {
            pos.iter()
                .map(|&start| {
                    let rest_start = start + group_len + 1;
                    if rest_start >= line.len() {
                        vec![]
                    } else {
                        let following = place_groups_unfiltered(&line[rest_start..], &groups[1..]);

                        following
                            .iter()
                            .map(|v| {
                                [vec![start], v.iter().map(|val| val + rest_start).collect()]
                                    .concat()
                            })
                            .collect()
                    }
                })
                .flatten()
                .collect()
        }
    } else {
        HashSet::new()
    }
}

// fn place_all_groups<'a>(line: &str, groups: &[usize]) -> HashSet<&'a Vec<usize>> {
//     place_groups_unfiltered(line, groups)
//         .iter()
//         .filter(|&placement| has_all_fixed(placement, groups, line))
//         .collect()
// }

// #[test]
// fn test_place_all_groups() {
//     let test_input = "?????";
//     let placements = place_all_groups(test_input, &[1, 1, 1]);
//     assert_eq!(placements, HashSet::from([vec![0, 2, 4]]));

//     let test_input = "?????";
//     let placements = place_all_groups(test_input, &[3, 1]);
//     assert_eq!(placements, HashSet::from([vec![0, 4]]));

//     let test_input = "?????";
//     let placements = place_all_groups(test_input, &[1]);
//     assert_eq!(
//         placements,
//         HashSet::from([vec![0], vec![1], vec![2], vec![3], vec![4]])
//     );

//     let test_input = "?????";
//     let placements = place_all_groups(test_input, &[1]);
//     assert_eq!(
//         placements,
//         HashSet::from([vec![0], vec![1], vec![2], vec![3], vec![4]])
//     );

//     let test_input = "???.#";
//     let placements = place_all_groups(test_input, &[1, 1]);
//     assert_eq!(
//         placements,
//         HashSet::from([vec![0, 4], vec![1, 4], vec![2, 4]])
//     );

//     let test_input = "#.#.?.###";
//     let placements = place_all_groups(test_input, &[1, 1, 3]);
//     assert_eq!(placements, HashSet::from([vec![0, 2, 6]]));
// }

fn render(placement: &[usize], groups: &[usize], linelen: usize) -> String {
    assert!(groups.len() == placement.len());

    let mut str = String::new();
    let mut len_sofar = 0;
    for (&pos, &len) in placement.iter().zip(groups.iter()) {
        for _ in 0..pos - len_sofar {
            str.push('.');
        }
        for _ in 0..len {
            str.push('#');
        }
        len_sofar += (pos - len_sofar) + len;
    }
    for _ in len_sofar..linelen {
        str.push('.');
    }

    str
}

fn main() {
    // let (folded_sum, unfolded_sum): (usize, usize) = lines
    let unfolded_sum: usize = stdio_lines_trimmed()
        .par_iter()
        .map(|line| {
            let (folded_line, groups) = line.split_at(line.find(' ').unwrap());
            let folded_groups: Vec<usize> = groups[1..]
                .split(',')
                .map(FromStr::from_str)
                .flatten()
                .collect();

            let unfolded_line: String = repeat([folded_line, "?"]).flatten().take(9).collect();
            let unfolded_groups: Vec<usize> =
                repeat(folded_groups.clone()).take(5).flatten().collect();

            // let folded_placements = place_all_groups(folded_line, &folded_groups);
            // let folded_count = folded_placements.len();
            // println!("folded: {folded_count}");

            let unfiltered = place_groups_unfiltered(&unfolded_line, &unfolded_groups);
            let unfolded_placements = unfiltered
                .iter()
                .filter(|&placement| has_all_fixed(placement, &unfolded_groups, &unfolded_line));
            let unfolded_count = unfolded_placements.count();
            println!("unfolded: {unfolded_count}");

            // for placement in folded_placements {
            //     println!("{}", render(&placement, &groups, line.len()));
            // }

            // (folded_count, unfolded_count)

            std::mem::drop(unfiltered);
            std::mem::drop(folded_groups);
            std::mem::drop(unfolded_line);
            std::mem::drop(unfolded_groups);

            unfolded_count
        })
        .sum();
    // .reduce(|| (0, 0), |l, r| (l.0 + r.0, l.1 + r.1));

    // println!("part1: {}", folded_sum);
    println!("part2: {}", unfolded_sum);
}
