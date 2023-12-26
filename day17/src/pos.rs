#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i32, pub i32);
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Pos(x as i32, y as i32)
    }
}
impl Into<(i32, i32)> for Pos {
    fn into(self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
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
impl std::ops::Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::AddAssign<(i32, i32)> for Pos {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0 += rhs.0;
        self.1 += rhs.1;
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
impl std::ops::Sub<(i32, i32)> for Pos {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::SubAssign<(i32, i32)> for Pos {
    fn sub_assign(&mut self, rhs: (i32, i32)) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
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
