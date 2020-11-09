pub mod sphere;

use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_record: &HitRecord) -> bool;
}
