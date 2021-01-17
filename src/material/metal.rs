use rand::prelude::ThreadRng;

use crate::vec3::RayPropagation;
use crate::{Color, HitRecord, Random, Ray, Vec3};

use super::{Attenuation, Material};

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Attenuation, Ray)> {
        let reflected = Vec3::reflect(&r.direction(), &rec.normal);
        let ray = Ray::new(rec.p, reflected + self.fuzz * Vec3::gen_in_unit_sphere());
        if ray.direction().dot(rec.normal) < 0.0 {
            ()
        }
        Some((self.albedo, ray))
    }
}
