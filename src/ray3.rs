use crate::vector3::{Point3, Vector3};

pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vector3) -> Ray3 {
        Ray3 {
            origin,
            direction
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}
