use math::vector::Vec3;
use math::vector::DotProduct;

pub fn normalize(mut v: Vec3<f32>) -> Vec3<f32> {
    let l = v.length();
    if l == 0.0 {
        v
    } else {
        v /= l;
        v
    }
}

pub fn slerp(v0: &Vec3<f32>, v1: &Vec3<f32>, t: f32) -> Vec3<f32> {
    let omega = v0.dot(v1).acos();
    ((v0 * ((1.0 - t) * omega).sin()) + (v1 * (t * omega).sin())) / omega.sin()
}

pub fn lerp(v0: &Vec3<f32>, v1: &Vec3<f32>, t: f32) -> Vec3<f32> {
    v0 * (1.0 - t) + v1 * t
}

pub fn distance(v0: &Vec3<f32>, v1: &Vec3<f32>) -> f32 {
    (v1 - v0).length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_vector() {
        let a = Vec3::new(1.0, 2.0, 2.0);
        let b = normalize(a.clone());

        assert_eq!(b, a / 3.0f32);
    }

    #[test]
    fn slerp_vectors() {
        let sqrt2 = 2.0f32.sqrt();
        // Pick two points on non-unit sphere
        let ref a = Vec3::new(2.0, 0.0, 0.0);
        let ref b = Vec3::new(0.0, 2.0, 0.0);

        assert_eq!(slerp(a, b, 0.0),
                   a.clone(),
                   "slerp(t = 0) should be at start point");
        assert_eq!(slerp(a, b, 1.0),
                   b.clone(),
                   "slerp(t = 1) should be at end point");

        assert_eq!(slerp(a, b, 0.5),
                   Vec3::new(sqrt2, sqrt2, 0.0),
                   "slerp(t = 0.5) should be between points");

    }
}
