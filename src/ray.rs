use crate::vec3::{Vec3, Point};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + t*self.direction
    }
}