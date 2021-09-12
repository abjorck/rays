use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else { -outward_normal };
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }

        let dsqrt = discriminant.sqrt();
        let mut root = (-half_b - dsqrt) / a;
        if root < t_min || root >= t_max {
            root = (-half_b + dsqrt) / a;
            if root < t_min || root >= t_max {
                return None;
            }
        }

        let mut rec: HitRecord = HitRecord::default();
        rec.t = root;
        rec.p = r.at(root);
        let normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, normal);

        Some(rec)

    }
}