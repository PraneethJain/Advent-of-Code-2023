use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl Vec3 {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    pub fn box_prod(a: Self, b: Self, c: Self) -> i128 {
        a.x * (b.y * c.z - b.z * c.y)
            + a.y * (b.z * c.x - b.x * c.z)
            + a.z * (b.x * c.y - b.y * c.x)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
