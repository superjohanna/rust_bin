use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct CoordinateU16 {
    pub x: u16,
    pub y: u16,
}

impl std::ops::Add for CoordinateU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<(i8, i8)> for CoordinateU16 {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self {
        Self {
            x: (self.x as i16 + x as i16) as u16,
            y: (self.y as i16 + y as i16) as u16,
        }
    }
}

impl std::ops::Sub for CoordinateU16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<(i8, i8)> for CoordinateU16 {
    type Output = Self;

    fn sub(self, (x, y): (i8, i8)) -> Self::Output {
        Self {
            x: (self.x as i16 - x as i16) as u16,
            y: (self.y as i16 - y as i16) as u16,
        }
    }
}

impl std::fmt::Display for CoordinateU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}
