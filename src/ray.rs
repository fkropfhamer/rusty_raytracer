use crate::{vec3, hittable::Hittable};


pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> vec3::Vec3 {
        self.origin + vec3::scale(t, self.direction)
    }

    pub fn origin(&self) -> vec3::Vec3 {
        self.origin
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.direction
    }
}

pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
    Ray {
        origin,
        direction,
    }
}

// fn hit_sphere(center: &vec3::Vec3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.origin - *center;
//     let a = vec3::length_squared(&r.direction);
//     let half_b = vec3::dot(oc, r.direction);
//     let c = vec3::length_squared(&oc) - radius * radius;
//     let disriminant = half_b * half_b - a * c;
//     if disriminant < 0.0 {
//         return -1.0; 
//     } else {
//         return (-half_b - disriminant.sqrt()) / a;
//     }
// }

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> vec3::Vec3 {
    let hit_record = world.hit(ray, 0.0, f64::INFINITY);
    match hit_record {
        Some(rec) => {
            return vec3::scale(0.5, rec.normal + vec3::new(1.0, 1.0, 1.0))
        }

        None => {
            let unit_direction = vec3::unit_vector(&ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            vec3::scale(1.0 - t, vec3::new(1.0, 1.0, 1.0)) + vec3::scale(t, vec3::new(0.5, 0.7, 1.0))
        }
    }


}

