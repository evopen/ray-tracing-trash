use crate::ray::Ray;
use crate::vec3::RayPropagation;
use crate::vec3::Vec3;
use crate::Color;

use super::{Attenuation, Material};

pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &crate::hittable::HitRecord) -> (Attenuation, Ray) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refract_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = r.direction().normalize();
        let refracted = Vec3::refract(&unit_dir, &rec.normal, refract_ratio);
        let scattered = Ray::new(rec.p, refracted);
        (attenuation, scattered)
    }
}
