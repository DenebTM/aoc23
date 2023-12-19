use std::{collections::HashSet, io, str::FromStr};

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

// #[derive(Clone, Copy, PartialEq, Eq)]
// enum GroupKind {
//     Ok,
//     Damaged,
//     Unknown,
// }

// impl From<char> for GroupKind {
//     fn from(value: char) -> Self {
//         match value {
//             '.' => Self::Ok,
//             '#' => Self::Damaged,
//             _ => Self::Unknown,
//         }
//     }
// }

// #[derive(Clone, Copy)]
// struct Group {
//     kind: GroupKind,
//     len: usize,
// }

// fn split_chars(line: &str) -> Vec<Group> {
//     let mut list: Vec<Group> = Vec::new();
//     let mut current: String = String::new();

//     let mut current = Group {
//         kind: GroupKind::Unknown,
//         len: 0,
//     };

//     for ch in line.chars() {
//         if current.len == 0 {
//             current.kind = ch.into();
//         } else if current.kind == ch.into() {
//             current.len += 1;
//         } else {
//             list.push(current);
//             current.len = 0;
//         }
//     }

//     list
// }

fn placements(line: &str, group_len: usize) -> HashSet<usize> {
    if line.len() < group_len {
        return HashSet::new();
    }

    let mut pos = HashSet::new();

    let mut start = 0;
    let mut known_len = 0;
    let mut unknown_len = 0;

    for ch in line.chars() {
        if known_len + unknown_len == group_len && matches!(ch, '.' | '?') {
            pos.insert(start);
            break;
        } else if ch == '#' {
            known_len += 1;
        } else if ch == '?' {
            unknown_len += 1;
        } else {
            start += known_len + unknown_len + 1;
            known_len = 0;
            unknown_len = 0;
        }
    }

    // finish up last element
    if known_len + unknown_len == group_len {
        pos.insert(start);
    }

    let mut next_index = 1;
    while line.chars().nth(next_index - 1) == Some('#') && next_index < line.len() {
        next_index += 1;
        if next_index >= line.len() {
            break;
        }
    }

    // found non-fixed group, try other combinations too
    if known_len != group_len {
        let other_pos = placements(&line[next_index..], group_len);
        pos.extend(other_pos.iter().map(|pos| pos + next_index));
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

    let test_input = "#####";
    let positions = placements(test_input, 4);
    assert_eq!(positions, HashSet::from([]));
}

fn place_all_groups(line: &str, groups: &[usize]) -> HashSet<Vec<usize>> {
    if let Some(&group_len) = groups.first() {
        let pos = placements(line, group_len);
        if groups.len() == 1 {
            pos.iter()
                .filter_map(|&p| {
                    let rest = &line[p + group_len..];

                    if rest.chars().all(|c| c != '#') {
                        Some(vec![p])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            pos.iter()
                .map(|&start| {
                    let rest_start = start + group_len + 1;
                    if rest_start >= line.len() {
                        vec![]
                    } else {
                        let following = place_all_groups(&line[rest_start..], &groups[1..]);

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

#[test]
fn test_place_all_groups() {
    let test_input = "?????";
    let placements = place_all_groups(test_input, &[1, 1, 1]);
    assert_eq!(placements, HashSet::from([vec![0, 2, 4]]));

    let test_input = "?????";
    let placements = place_all_groups(test_input, &[3, 1]);
    assert_eq!(placements, HashSet::from([vec![0, 4]]));

    let test_input = "?????";
    let placements = place_all_groups(test_input, &[1]);
    assert_eq!(
        placements,
        HashSet::from([vec![0], vec![1], vec![2], vec![3], vec![4]])
    );

    let test_input = "?????";
    let placements = place_all_groups(test_input, &[1]);
    assert_eq!(
        placements,
        HashSet::from([vec![0], vec![1], vec![2], vec![3], vec![4]])
    );

    let test_input = "???.#";
    let placements = place_all_groups(test_input, &[1, 1]);
    assert_eq!(
        placements,
        HashSet::from([vec![0, 4], vec![1, 4], vec![2, 4]])
    );
}

fn main() {
    let sum: usize = stdio_each(|line, _| {
        let (line, groups) = line.trim().split_at(line.find(' ').unwrap());
        let groups: Vec<usize> = groups[1..]
            .split(',')
            .map(FromStr::from_str)
            .flatten()
            .collect();

        let placements = place_all_groups(line, &groups);
        let count = placements.len();

        for placement in placements {
            let mut linelen = 0;
            for (&pos, &len) in placement.iter().zip(groups.iter()) {
                for _ in 0..pos - linelen {
                    print!(".");
                }
                for _ in 0..len {
                    print!("#");
                }
                linelen += (pos - linelen) + len;
            }
            for _ in linelen..line.len() {
                print!(".");
            }
            println!();
        }
        println!("{count}");

        count
    })
    .iter()
    .sum();

    println!("part1: {}", sum);
}
