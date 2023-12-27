use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use crate::{
    grid::Grid,
    pos::{Dir, Pos},
};

/**
 * Perhaps a better heuristic can be found?
 */
#[inline(always)]
fn heur(pos: Pos, end: Pos) -> i32 {
    ((end.0 - pos.0).abs() + (end.1 - pos.1).abs()) as i32
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub pos: Pos,
    // pub pred: Option<Rc<Path>>,
    pub total_cost: i32,

    last_dir: Dir,
    straight_line: i32,

    end: Pos,
}

/// this ordering is reversed so that in a priority queue, the path with the lowest cost is returned first
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.total_cost + heur(self.pos, self.end))
            .partial_cmp(&(other.total_cost + heur(other.pos, other.end)))
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
            pos: start,
            // pred: None,
            total_cost: 0,
            last_dir: Dir::EAST,
            straight_line: 0,
            end,
        }
    }

    /**
     * TODO: This kind of works, but shows that A* is not designed for dynamic weights
     *   1. Slow due to lack of memorization
     *   2. Memorization does not take distance since last turn into account, so may miss more
     *      optimal solutions
     *
     * Consider 4D positions:
     *   x, z = 2D position
     *   y, w = entering direction and how long the path has gone in a straight line
     *
     * Based on this, we can build a static weighted graph for A* to work on
     * This will likely _also_ be slow though? As having 12 (4 [NESW] * 3 [distance since last turn])
     * copies of each 2D position will make memorization less effective.
     *
     * Idea for >2D position from: https://stackoverflow.com/a/52187896
     *
     * Also: at this point i should really create a graph datastructure
     */
    fn successors<'a>(
        self,
        grid: &'a Grid,
        closed_set: &'a mut HashMap<Pos, i32>,
    ) -> impl Iterator<Item = Path> + 'a {
        let s = Rc::new(self);
        grid.neighbours(s.pos)
            .filter_map(move |(next_dir, next_pos, &next_cost)| {
                let next_cost = s.total_cost + next_cost as i32;

                // don't go in the same direction for more than three tiles,
                // and don't go backwards either
                if (closed_set.contains_key(&next_pos)
                    && *closed_set.get(&next_pos).unwrap() <= next_cost - 2) // cost-2 => hack to get things working with the test input
                    || -next_dir == s.last_dir
                    || (s.straight_line >= 3 && next_dir == s.last_dir)
                {
                    None
                } else {
                    closed_set.insert(next_pos, next_cost);
                    Some(Self {
                        pos: next_pos,
                        // pred: Some(s.clone()),
                        total_cost: next_cost,
                        last_dir: next_dir,
                        straight_line: if next_dir == s.last_dir {
                            s.straight_line + 1
                        } else {
                            1
                        },
                        end: s.end,
                    })
                }
            })
    }
}

pub fn a_star(grid: &Grid, start: Pos, end: Pos) -> Option<Path> {
    let init_path = Path::new(start, end);
    let mut open_set = BinaryHeap::from([init_path]);
    let mut closed_set = HashMap::from([(start, 0)]);

    let mut found_path: Option<Path> = None;
    while open_set.len() > 0 {
        let path = open_set.pop().unwrap();
        if path.pos == end {
            found_path = Some(path);
            break;
        }

        for succ in path.successors(grid, &mut closed_set) {
            open_set.push(succ)
        }
    }

    found_path
}
