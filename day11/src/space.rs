use std::collections::{HashMap, HashSet};

type GalaxyNum = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Galaxy {
    pub num: GalaxyNum,
    pub pos: (usize, usize),
}

#[derive(Clone)]
pub struct Space {
    pub empty_factor: usize,

    pub galaxies: HashMap<GalaxyNum, Galaxy>,
    pub last_galaxy: GalaxyNum,

    pub filled_rows: HashSet<usize>,
    pub filled_cols: HashSet<usize>,
}

impl Space {
    pub fn new(empty_factor: usize) -> Self {
        Self {
            empty_factor,
            galaxies: HashMap::new(),
            last_galaxy: 0,
            filled_rows: HashSet::new(),
            filled_cols: HashSet::new(),
        }
    }

    pub fn from(lines: Vec<String>, empty_factor: usize) -> Self {
        let mut space = Self::new(empty_factor);

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                match lines[y].chars().nth(x) {
                    Some('#') => space.add_galaxy((x, y)),
                    _ => {}
                }
            }
        }

        space
    }

    pub fn add_galaxy(&mut self, pos: (usize, usize)) {
        self.last_galaxy += 1;

        self.galaxies.insert(
            self.last_galaxy,
            Galaxy {
                num: self.last_galaxy,
                pos,
            },
        );

        self.filled_cols.insert(pos.0);
        self.filled_rows.insert(pos.1);
    }

    pub fn get_gal_pairs(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let a = (1..=self.last_galaxy)
            .map(|num1| (num1 + 1..=self.last_galaxy).map(move |num2| (num1, num2)))
            .flatten();

        a
    }

    fn get_expanded_dist_x(&self, mut x1: usize, mut x2: usize) -> usize {
        if x2 < x1 {
            std::mem::swap(&mut x1, &mut x2);
        }

        let empty_cols_count = (x1..x2).filter(|x| !self.filled_cols.contains(x)).count();

        (x2 - x1 - empty_cols_count) + empty_cols_count * self.empty_factor
    }
    fn get_expanded_dist_y(&self, mut y1: usize, mut y2: usize) -> usize {
        if y2 < y1 {
            std::mem::swap(&mut y1, &mut y2);
        }

        let empty_rows_count = (y1..y2).filter(|x| !self.filled_rows.contains(x)).count();

        (y2 - y1 - empty_rows_count) + empty_rows_count * self.empty_factor
    }

    pub fn get_expanded_dist(&self, gal1: &Galaxy, gal2: &Galaxy) -> usize {
        self.get_expanded_dist_x(gal1.pos.0, gal2.pos.0)
            + self.get_expanded_dist_y(gal1.pos.1, gal2.pos.1)
    }
}
