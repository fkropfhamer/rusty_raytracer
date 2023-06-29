use crate::hittable::{self, HitRecord};

pub struct HittableList {
    pub objects: Vec<Box<dyn hittable::Hittable>>
}


impl hittable::Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let o = object.hit(ray, t_min, closest_so_far);
            match temp_rec {
                Some(rec) => {
                    closest_so_far = rec.t
                }
                None => {}
            }
            temp_rec = o;

        }

        return temp_rec
    }
}
