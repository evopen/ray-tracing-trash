use rand::prelude::ThreadRng;

use crate::{Color, HitRecord, Random, Ray, Vec3};

use super::{Attenuation, Material};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (Attenuation, Ray) {
        let mut rng = rand::thread_rng();
        let scatter_direction = rec.normal + Vec3::gen_unit_vector(&mut rng);
        let ray = Ray::new(rec.p, rec.normal + scatter_direction);
        return (self.albedo, ray);
    }
}
