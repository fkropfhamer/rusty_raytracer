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

pub fn ray_color(ray: Ray) -> vec3::Vec3 {
    let unit_direction = vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.get_y() + 1.0);

    vec3::scale(1.0 - t, vec3::init(1.0, 1.0, 1.0)) + vec3::scale(t, vec3::init(0.5, 0.7, 1.0))
}

