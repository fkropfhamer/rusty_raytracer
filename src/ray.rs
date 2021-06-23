use crate::vec3;


pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3
}

impl Ray {
    fn at(&self, t: f64) -> vec3::Vec3 {
        self.origin + vec3::scale(t, self.direction)
    }
}

pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
    Ray {
        origin,
        direction,
    }
}

fn hit_sphere(center: &vec3::Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = vec3::dot(r.direction, r.direction);
    let b = 2.0 * vec3::dot(oc, r.direction);
    let c = vec3::dot(oc, oc) - radius * radius;
    let disriminant = b * b - 4.0 * a * c;
    if disriminant < 0.0 {
        return -1.0; 
    } else {
        return (-b - disriminant.sqrt()) / (2.0 * a);
    }
}

pub fn ray_color(ray: &Ray) -> vec3::Vec3 {
    let mut t = hit_sphere(&vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = vec3::unit_vector(&(ray.at(t) - vec3::new(0.0, 0.0, -1.0)));
        return vec3::scale(0.5, vec3::new(n.get_x() + 1.0, n.get_y() + 1.0, n.get_z() + 1.0));
    }
    let unit_direction = vec3::unit_vector(&ray.direction);
    t = 0.5 * (unit_direction.get_y() + 1.0);

    vec3::scale(1.0 - t, vec3::new(1.0, 1.0, 1.0)) + vec3::scale(t, vec3::new(0.5, 0.7, 1.0))
}

