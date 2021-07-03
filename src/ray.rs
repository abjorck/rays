use crate::vec3::{Vec3, Point, Color};

pub struct Ray {
    pub orig: Point,
    pub dir: Vec3
}

impl Ray {
    pub fn at(self, t: f32) -> Point {
        self.orig + t*self.dir
    }
}