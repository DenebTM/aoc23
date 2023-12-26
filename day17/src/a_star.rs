use std::{collections::BinaryHeap, rc::Rc};

use crate::{
    grid::Grid,
    pos::{Dir, Pos},
};

fn heur(pos: Pos, end: Pos) -> usize {
    ((end.0 - pos.0).abs() + (end.1 - pos.1).abs()) as usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub pos: Pos,
    pub pred: Option<Rc<Path>>,
    pub total_cost: usize,

    last_dir: Option<Dir>,
    straight_line: usize,

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
            pred: None,
            total_cost: 0,
            last_dir: None,
            straight_line: 0,
            end,
        }
    }

    fn successors(self, grid: &Grid) -> impl Iterator<Item = Path> + '_ {
        let s = Rc::new(self);
        grid.neighbours(s.pos)
            .filter_map(move |(next_dir, next_pos, &next_cost)| {
                let s = s.clone();

                // don't go in the same direction for more than three tiles,
                // and don't go backwards either
                if Some(-next_dir) == s.last_dir
                    || Some(next_dir) == s.last_dir && s.straight_line >= 3
                {
                    None
                } else {
                    Some(Self {
                        pos: next_pos,
                        pred: Some(s.clone()),
                        total_cost: s.total_cost + next_cost as usize,
                        last_dir: Some(next_dir),
                        straight_line: if Some(next_dir) == s.last_dir {
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

    let mut found_path: Option<Path> = None;
    while open_set.len() > 0 {
        let path = open_set.pop().unwrap();
        if path.pos == end {
            found_path = Some(path);
            break;
        }

        for succ in path.successors(grid) {
            open_set.push(succ)
        }
    }

    found_path
}
