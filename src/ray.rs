use crate::vec3;


pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3
}

pub fn init(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
    Ray {
        origin,
        direction,
    }
}

fn hit_sphere(center: &vec3::Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = vec3::dot(r.direction, r.direction);
    let b = 2.0 * vec3::dot(oc, r.direction);
    let c = vec3::dot(oc, oc) - radius * radius;
    let disriminant = b * b - 4.0 * a * c;
    disriminant > 0.0
}

pub fn ray_color(ray: Ray) -> vec3::Vec3 {
    if hit_sphere(&vec3::init(0.0, 0.0, -1.0), 0.5, &ray) {
        return vec3::init(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.get_y() + 1.0);

    vec3::scale(1.0 - t, vec3::init(1.0, 1.0, 1.0)) + vec3::scale(t, vec3::init(0.5, 0.7, 1.0))
}

