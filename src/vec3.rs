use std::ops::Add;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn to_color_string(&self) -> String {
        let ir = (255.999 * self.x) as i64;
        let ig = (255.999 * self.y) as i64;
        let ib = (255.999 * self.z) as i64;

        format!("{} {} {} \n", ir, ig, ib)
    }

    pub fn to_string(&self) -> String {
        format!("Vec3 x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        add(self, other)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        subtract(self, other)
    }
}

pub fn init(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 {
        x,
        y,
        z,
    }
}

pub fn scale(s: f64, v: Vec3) -> Vec3 {
    init(s * v.x, s * v.y, s * v.z)
}

pub fn add(vec1: Vec3, vec2: Vec3) -> Vec3 {
    let x = vec1.x + vec2.x;
    let y = vec1.y + vec2.y;
    let z = vec1.z + vec2.z;

    return Vec3 {
        x,
        y,
        z,
    };
}

pub fn subtract(vec1: Vec3, vec2: Vec3) -> Vec3 {
    let x = vec1.x - vec2.x;
    let y = vec1.y - vec2.y;
    let z = vec1.z - vec2.z;

    Vec3 {
        x,
        y,
        z,
    }
}

pub fn dot(vec1: Vec3, vec2: Vec3) -> f64 {
    return vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
}

pub fn length_squared(v: &Vec3) -> f64 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

pub fn length(v: &Vec3) -> f64 {
    length_squared(v).sqrt()
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let l = length(v);
    let x = v.x / l;
    let y = v.y / l;
    let z = v.z / l;

    Vec3 {
        x,
        y,
        z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec2 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let vec3 = Vec3 {
            x: 2.0,
            y: 4.0,
            z: 6.0,
        };

        assert_eq!(add(vec1, vec2), vec3)
    }

    #[test]
    fn test_init() {
        let vec = init(1.0, 2.0, 3.0);

        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_dot() {
        let vec1 = init(1.0, 2.0, 3.0);
        let vec2 = init(1.0, 2.0, 3.0);

        assert_eq!(dot(vec1, vec2), 14.0)
    }
}
