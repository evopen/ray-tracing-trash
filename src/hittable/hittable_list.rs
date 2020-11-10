use super::{HitRecord, Hittable};

struct HittableList {
    objects: Vec<Box<Hittable>>,
}

impl HittableList {
    fn new(objects: Vec<Box<Hittable>>) -> Self {
        Self { objects }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Box<Hittable>) {
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
