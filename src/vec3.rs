#[derive(PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub fn init(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 {
        x,
        y,
        z,
    }
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

pub fn dot(vec1: Vec3, vec2: Vec3) -> f32 {
    return vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
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
