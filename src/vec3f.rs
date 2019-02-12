#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(test)]
impl Arbitrary for Vec3f {
    fn arbitrary<G: Gen>(g: &mut G) -> Vec3f {
        let x = f32::arbitrary(g);
        let y = f32::arbitrary(g);
        let z = f32::arbitrary(g);
        Vec3f { x, y, z }
    }
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn from_one(x: f32) -> Vec3f {
        Vec3f::new(x, x, x)
    }

    pub fn zero() -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }

    pub fn unit() -> Vec3f {
        Vec3f::new(1.0, 1.0, 1.0)
    }

    pub fn unit_forward() -> Vec3f {
        Vec3f::new(0.0, 0.0, 1.0)
    }

    pub fn unit_backward() -> Vec3f {
        Vec3f::new(0.0, 0.0, -1.0)
    }

    pub fn unit_right() -> Vec3f {
        Vec3f::new(1.0, 0.0, 0.0)
    }

    pub fn unit_left() -> Vec3f {
        Vec3f::new(-1.0, 0.0, 0.0)
    }

    pub fn unit_up() -> Vec3f {
        Vec3f::new(0.0, 1.0, 0.0)
    }

    pub fn unit_down() -> Vec3f {
        Vec3f::new(0.0, -1.0, 0.0)
    }

    pub fn magnitude(&self) -> f32 {
        let res = self.dot(&*self).sqrt();
        debug_assert!(res + 1e-5 >= 0.0);
        res
    }

    pub fn squared_magnitude(&self) -> f32 {
        let res = self.dot(&*self);
        debug_assert!(res + 1e-5 >= 0.0);
        res
    }

    pub fn normalized(&self) -> Vec3f {
        let length = self.magnitude();
        let res = Vec3f::new(self.x / length, self.y / length, self.z / length);
        debug_assert!((res.squared_magnitude() - 1.0).abs() <= 1e-5);
        res
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn comp(self, other: &Vec3f) -> f32 {
        self.dot(other) / self.magnitude()
    }

    pub fn project(self, other: &Vec3f) -> Vec3f {
        (self.dot(other) / self.squared_magnitude()) * self
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, other: f32) -> Vec3f {
        Vec3f::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, other: Vec3f) -> Vec3f {
        other * self
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}

impl Div for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}
