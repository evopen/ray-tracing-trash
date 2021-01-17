pub use glam::Vec3A as Vec3;

use glam::const_vec3a;
use rand::prelude::*;

const MAX_U64: f32 = std::u64::MAX as f32;

pub trait Random {
    fn gen() -> Self;
    fn gen_range(rnmin: f32, max: f32) -> Self;
    fn gen_in_unit_sphere() -> Self;
    fn gen_unit_vector() -> Self;
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

pub fn random() -> u64 {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0;
        STATE = STATE
            .wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        STATE
    }
}

impl Random for Vec3 {
    fn gen() -> Self {
        Vec3::gen_range(0.0, 1.0)
    }

    fn gen_range(min: f32, max: f32) -> Self {
        let max_u64: Vec3 = Vec3::new(MAX_U64, MAX_U64, MAX_U64);
        let range = max - min;
        let x = random() as f32;
        let y = random() as f32;
        let z = random() as f32;
        Vec3::new(x, y, z) / max_u64 * Vec3::new(range, range, range) + Vec3::new(min, min, min)
    }

    fn gen_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::gen();
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    fn gen_unit_vector() -> Self {
        Vec3::gen_in_unit_sphere().normalize()
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
