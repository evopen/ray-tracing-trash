mod lambertian;
mod metal;

use crate::{Color, HitRecord, Ray, Vec3};
pub use lambertian::Lambertian;
pub use metal::Metal;

pub type Attenuation = Vec3;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Attenuation, Ray)>;
}

impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a material")
    }
}
