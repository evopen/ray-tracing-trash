use rand::prelude::ThreadRng;

use crate::vec3::RayPropagation;
use crate::{Color, HitRecord, Random, Ray, Vec3};

use super::{Attenuation, Material};

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self { Self { albedo } }
}


impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (Attenuation, Ray) {
        let reflected = Vec3::reflect(&r.direction(), &rec.normal);
        let ray = Ray::new(rec.p, reflected);
        if ray.direction().dot(rec.normal) < 0.0 {
            panic!("what?")
        }
        (self.albedo, ray)
    }
}
