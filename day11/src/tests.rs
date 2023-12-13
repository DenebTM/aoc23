#![allow(unused_imports, dead_code)]

/// TDD wow!!!
use std::collections::HashSet;

use crate::space::{Galaxy, Space};

static TEST_INPUT: [&str; 10] = [
    "...#......",
    ".......#..",
    "#.........",
    "..........",
    "......#...",
    ".#........",
    ".........#",
    "..........",
    ".......#..",
    "#...#.....",
];

#[test]
fn test_get_gal_pairs() {
    let test_input = TEST_INPUT.iter().map(|s| s.to_string()).collect();
    let space = Space::from(test_input, 2);

    let expected_output = HashSet::from([
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (1, 6),
        (1, 7),
        (1, 8),
        (1, 9),
        (2, 3),
        (2, 4),
        (2, 5),
        (2, 6),
        (2, 7),
        (2, 8),
        (2, 9),
        (3, 4),
        (3, 5),
        (3, 6),
        (3, 7),
        (3, 8),
        (3, 9),
        (4, 5),
        (4, 6),
        (4, 7),
        (4, 8),
        (4, 9),
        (5, 6),
        (5, 7),
        (5, 8),
        (5, 9),
        (6, 7),
        (6, 8),
        (6, 9),
        (7, 8),
        (7, 9),
        (8, 9),
    ]);

    let gal_pair_nums: HashSet<(usize, usize)> = space.get_gal_pairs().collect();

    for (gal1, gal2) in expected_output {
        assert!(gal_pair_nums.contains(&(gal1, gal2)) || gal_pair_nums.contains(&(gal2, gal1)));
        assert!(!(gal_pair_nums.contains(&(gal1, gal2)) && gal_pair_nums.contains(&(gal2, gal1))));
    }
}

#[test]
fn test_get_expanded_dist() {
    let test_input = vec!["#..".to_string(), "...".to_string(), "..#".to_string()];
    let mut space = Space::from(test_input, 2);

    let gal_vec: Vec<&Galaxy> = space.galaxies.iter().map(|(_, gal)| gal).collect();

    space.empty_factor = 2;
    assert!(space.get_expanded_dist(gal_vec[0], gal_vec[1]) == 6);

    space.empty_factor = 3;
    assert!(space.get_expanded_dist(gal_vec[0], gal_vec[1]) == 8);
}
