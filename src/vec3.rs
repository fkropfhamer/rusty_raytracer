#[derive(PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
}
