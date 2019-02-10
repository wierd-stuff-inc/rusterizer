use crate::Vec3f;
#[cfg(test)]
extern crate quickcheck;

#[quickcheck]
fn magnitude(x: Vec3f) -> bool {
    x.magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn squared_magnitude(x: Vec3f) -> bool {
    x.squared_magnitude() + 1e-6 >= 0.0
}

#[quickcheck]
fn normalized(x: Vec3f) -> bool {
    (x.normalized().magnitude() - 1.0).abs() <= 1e-6
}
