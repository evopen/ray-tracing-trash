use crate::hittable::{HitRecord, Hittable};
use crate::{Color, Point3, Vec3};
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = Vec3::from(r.origin() - self.center);
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.center).normalize();
        let mut rec = HitRecord {
            p: hit_point,
            normal: outward_normal,
            t: root,
            front_face: false,
        };

        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}
