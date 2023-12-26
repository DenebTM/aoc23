use crate::pos::{Dir, Pos};

pub struct Grid {
    pub data: Vec<Vec<usize>>,
}
impl From<Vec<String>> for Grid {
    fn from(mat: Vec<String>) -> Self {
        let mut data = Vec::new();

        for line in mat.iter() {
            let mut data_line = Vec::new();
            for ch in line.chars() {
                data_line.push(ch.to_string().parse::<usize>().unwrap())
            }
            data.push(data_line);
        }

        Self { data }
    }
}
impl Grid {
    pub fn width(&self) -> usize {
        self.data[0].len()
    }
    pub fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, pos: Pos) -> Option<&usize> {
        self.data
            .get(pos.1 as usize)
            .map(|line| line.get(pos.0 as usize))
            .flatten()
    }

    pub fn neighbours(&self, pos: Pos) -> impl Iterator<Item = (Dir, Pos, &usize)> + '_ {
        [Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST]
            .iter()
            .filter_map(move |&dir| Some((dir, pos + dir, self.get(pos + dir)?)))
    }
}
