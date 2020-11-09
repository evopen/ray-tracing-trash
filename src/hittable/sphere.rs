use crate::hittable::{HitRecord, Hittable};
use crate::{Color, Point3, Vec3};
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = Vec3::from(r.origin() - self.center);
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
    }
}
