use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use crate::{
    grid::Grid,
    pos::{Dir, Pos},
};

#[inline(always)]
fn heur(pos: Pos, end: Pos) -> i32 {
    ((end.0 - pos.0).abs() + (end.1 - pos.1).abs()) as i32 * 2
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    current: Pos,               // current tip of path
    pub prev: Option<Rc<Path>>, // allow reconstruction of the path
    pub total_cost: i32,        // current path cost
    last_dir: Dir,              // direction path last took
    straight_line: i32,         // how long path has been going in the same direction
    end: Pos,                   // final destination
}

/// this ordering is reversed so that in a priority queue, the path with the lowest cost is returned first
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.total_cost + heur(self.current, self.end))
            .partial_cmp(&(other.total_cost + heur(other.current, other.end)))
            .map(|ord| ord.reverse())
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Path {
    fn new(start: Pos, end: Pos) -> Self {
        Self {
            current: start,
            prev: None,
            total_cost: 0,
            last_dir: Dir::EAST,
            straight_line: 0,
            end,
        }
    }

    fn successors<'a>(
        self,
        grid: &'a Grid,
        closed_set: &'a mut HashMap<(Pos, Dir, i32), i32>,
        min_straight: i32,
        max_straight: i32,
    ) -> impl Iterator<Item = Path> + 'a {
        let s = Rc::new(self);
        grid.neighbours(s.current)
            .filter_map(move |(next_dir, next_pos, &next_cost)| {
                let next_cost = s.total_cost + next_cost as i32;
                let next_straight_line = if next_dir == s.last_dir {
                    s.straight_line + 1
                } else {
                    1
                };

                let closed_set_key = (next_pos, next_dir, next_straight_line);

                // don't revisit tiles, keep in mind the approach direction and straight line length
                if (closed_set.contains_key(&closed_set_key)
                    && *closed_set.get(&closed_set_key).unwrap() <= next_cost)
                    // don't go backwards
                    || -next_dir == s.last_dir
                    // part2: go in the same direction for at least 4 tiles
                    || (s.straight_line < min_straight && next_dir != s.last_dir)
                    // part1: don't go in the same direction for more than 3 tiles (part2: 10)
                    || (s.straight_line >= max_straight && next_dir == s.last_dir)
                {
                    None
                } else {
                    closed_set.insert(closed_set_key, next_cost);
                    Some(Self {
                        current: next_pos,
                        prev: Some(s.clone()),
                        total_cost: next_cost,
                        last_dir: next_dir,
                        straight_line: next_straight_line,
                        end: s.end,
                    })
                }
            })
    }
}

/**
 * `closed_set` performs "4D" memorization -- we remember:
 * - if we've been at a position (x, y)
 * - from a certain direction
 * - after going in a straight line for 1, 2 or 3 steps
 *
 * This effectively turns the path costs static as far as A* is concerned.
 *
 * Partial credit for the idea goes to https://stackoverflow.com/a/52187896
 */
pub fn a_star(
    grid: &Grid,
    start: Pos,
    end: Pos,
    min_straight: i32,
    max_straight: i32,
) -> Option<Path> {
    let init_path = Path::new(start, end);
    let mut open_set = BinaryHeap::from([init_path]);
    let mut closed_set = HashMap::from([((start, Dir::EAST, 0), 0)]);

    let mut found_path: Option<Path> = None;
    while open_set.len() > 0 {
        let path = open_set.pop().unwrap();
        if path.current == end {
            found_path = Some(path);
            break;
        }

        for succ in path.successors(grid, &mut closed_set, min_straight, max_straight) {
            open_set.push(succ)
        }
    }

    found_path
}
