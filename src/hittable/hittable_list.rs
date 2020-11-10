use super::{HitRecord, Hittable};
use std::sync::Arc;


#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}


impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::default();

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = hit_record.t;
                *hit_record = temp_rec;
            }
        }
        return hit_anything;
    }
}
