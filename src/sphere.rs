use crate::vec3;
use crate::hittable;
use crate::ray;

pub struct Sphere {
  center: vec3::Vec3,
  radius: f64,
}

impl hittable::Hittable for Sphere {
  fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, mut rec: &mut hittable::HitRecord) -> bool {
    let oc = ray.origin() - self.center;
    let a = ray.direction().length_squared();
    let half_b = vec3::dot(oc, ray.direction());
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return false;
      } 
    }

    rec.t = root;
    rec.p = ray.at(rec.t);
    let outward_normal = vec3::scale(1.0 / self.radius, rec.p - self.center);
    rec.set_face_normal(ray, &outward_normal);

    true
  }
}

pub fn new(center: vec3::Vec3, radius: f64) -> Sphere {
  Sphere {
    center,
    radius,
  }
}
