#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub isize, pub isize);
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Pos(x as isize, y as isize)
    }
}
impl Into<(usize, usize)> for Pos {
    fn into(self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<'a> std::ops::Add for &'a Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl std::ops::Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Neg for Pos {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
impl<'a> std::ops::Neg for &'a Pos {
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Pos(-self.0, -self.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use std::ops::Add;
        self.add(-rhs)
    }
}
impl<'a> std::ops::Sub for &'a Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        use std::ops::Add;
        self.clone().add(-rhs.clone())
    }
}

pub type Dir = Pos;
impl Dir {
    pub const NORTH: Dir = Pos(0, -1);
    pub const NORTHEAST: Dir = Pos(1, -1);
    pub const EAST: Dir = Pos(1, 0);
    pub const SOUTHEAST: Dir = Pos(1, 1);
    pub const SOUTH: Dir = Pos(0, 1);
    pub const SOUTHWEST: Dir = Pos(-1, 1);
    pub const WEST: Dir = Pos(-1, 0);
    pub const NORTHWEST: Dir = Pos(-1, -1);
}
