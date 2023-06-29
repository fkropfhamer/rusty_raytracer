use crate::vec3;
use crate::ray;

pub struct HitRecord {
  pub p: vec3::Vec3,
  pub normal: vec3::Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: &vec3::Vec3) {
    self.front_face = vec3::dot(ray.direction(), *outward_normal) < 0.0;
    self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
  }
}


pub trait Hittable {
  fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

