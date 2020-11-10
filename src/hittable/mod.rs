mod hittable_list;
mod sphere;

use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub use sphere::Sphere;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
