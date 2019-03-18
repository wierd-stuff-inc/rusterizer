use nalgebra::Vector2;
use nalgebra::Vector3;

pub type Color = (u8, u8, u8);
pub type Point = Vector2<f64>;
pub type Line = (Point, Point);
pub type Vec3f = Vector3<f64>;
#[allow(dead_code)]
pub type Vec3i = Vector3<i32>;
