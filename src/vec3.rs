pub use glam::Vec3A as Vec3;

use rand::prelude::*;
pub trait Random {
    fn gen(rng: &mut ThreadRng) -> Self;
    fn gen_range(rng: &mut ThreadRng, min: f32, max: f32) -> Self;
    fn gen_in_unit_sphere(rng: &mut ThreadRng) -> Self;
    fn gen_unit_vector(rng: &mut ThreadRng) -> Self;
}

pub trait RayPropagation {
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3;
}

impl RayPropagation for Vec3 {
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        debug_assert!(n.is_normalized());
        return v.clone() - 2.0 * v.dot(n.clone()) * n.clone();
    }
}

impl Random for Vec3 {
    fn gen(rng: &mut ThreadRng) -> Self {
        Vec3::gen_range(rng, 0.0, 1.0)
    }

    fn gen_range(rng: &mut ThreadRng, min: f32, max: f32) -> Self {
        Vec3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    fn gen_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Vec3::gen(rng);
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    fn gen_unit_vector(rng: &mut ThreadRng) -> Self {
        Vec3::gen_in_unit_sphere(rng).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> ThreadRng {
        let _ = env_logger::builder().is_test(true).try_init();
        rand::thread_rng()
    }

    #[test]
    fn test_gen() {
        let mut rng = init();
        let result: [f32; 3] = Vec3::gen(&mut rng).into();
        for e in &result {
            assert!(e.to_owned() > 0.0 && e.to_owned() < 1.00);
        }
    }

    #[test]
    fn test_gen_range() {
        let mut rng = init();
        let result = Vec3::gen_range(&mut rng, 50.0, 200.0);
        for e in result.as_ref() {
            assert!(e.to_owned() >= 50.0 && e.to_owned() <= 200.0);
        }
    }

    #[test]
    fn test_gen_in_unit_sphere() {
        let mut rng = init();
        for _ in 0..100000 {
            let result = Vec3::gen_in_unit_sphere(&mut rng);
            for e in result.as_ref() {
                let e = e.to_owned();
                assert!(e >= 0.0 && e <= 1.0);
            }
        }
    }
}
