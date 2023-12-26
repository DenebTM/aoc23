use std::collections::{BinaryHeap, LinkedList};

use crate::{
    grid::Grid,
    pos::{Dir, Pos},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pos: Pos,
    cost: u8,
}

/// this ordering is reversed so that in a priority queue, the node with the lowest cost is returned first
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|ord| ord.reverse())
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn heur(pos: Pos, end: Pos) -> usize {
    ((end.0 - pos.0).abs() + (end.1 - pos.1).abs()) as usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path {
    pub path: LinkedList<Node>,
    pub total_cost: usize,

    last_dir: Option<Dir>,
    straight_line: usize,

    end: Pos,
}

/// this ordering is reversed so that in a priority queue, the path with the lowest cost is returned first
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.total_cost + heur(self.last().pos, self.end))
            .partial_cmp(&(other.total_cost + heur(other.last().pos, other.end)))
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
            path: [Node {
                pos: start,
                cost: 0,
            }]
            .into(),
            total_cost: 0,
            last_dir: None,
            straight_line: 0,
            end,
        }
    }

    fn last(&self) -> &Node {
        self.path.back().unwrap()
    }

    fn successors(&self, grid: &Grid) -> Vec<Path> {
        grid.neighbours(self.last().pos)
            .filter_map(|(next_dir, next_pos, next_cost)| {
                let mut path = self.path.clone();
                path.push_back(Node {
                    pos: next_pos,
                    cost: next_cost,
                });

                if Some(next_dir) == self.last_dir && self.straight_line >= 3 {
                    None
                } else {
                    Some(Self {
                        path,
                        total_cost: self.total_cost + next_cost as usize,
                        last_dir: Some(next_dir),
                        straight_line: if Some(next_dir) == self.last_dir {
                            self.straight_line + 1
                        } else {
                            1
                        },

                        end: self.end,
                    })
                }
            })
            .collect()
    }
}

pub fn a_star(grid: &Grid, start: Pos, end: Pos) -> Option<Path> {
    let init_path = Path::new(start, end);
    let mut open_set = BinaryHeap::from([init_path]);

    let mut found_path: Option<Path> = None;
    while open_set.len() > 0 {
        let path = open_set.pop().unwrap();
        if path.last().pos == end {
            found_path = Some(path);
            break;
        }

        for succ in path.successors(grid) {
            open_set.push(succ)
        }
    }

    found_path
}
