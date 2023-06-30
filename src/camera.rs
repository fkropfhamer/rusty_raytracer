use crate::{vec3, ray};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3
}


impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vec3::new(0.0, 0.0, 0.0);
        let horizontal = vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - vec3::scale(0.5, horizontal) - vec3::scale(0.5, vertical) - vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::new(self.origin, self.lower_left_corner + vec3::scale(u, self.horizontal) + vec3::scale(v, self.vertical) - self.origin)
    }
}

