use std::ops::*;
use na::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn to_point3(&self) -> Point3<f32> {
        Point3::new(self.data[0], self.data[1], self.data[2])
    }

    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.data[0], self.data[1], self.data[2])
    }

    pub fn length(&self) -> f32 {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2])
            .sqrt()
    }

    pub fn normal(&self) -> Vec3 {
        let l = self.length();
        if l == 0.0 { *self } else { *self / l }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] + other.data[0],
                  self.data[1] + other.data[1],
                  self.data[2] + other.data[2])
    }
}

impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3::new(self.data[0] + other.data[0],
                  self.data[1] + other.data[1],
                  self.data[2] + other.data[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.data[0] - other.data[0],
                  self.data[1] - other.data[1],
                  self.data[2] - other.data[2])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.data[0] * _rhs,
                  self.data[1] * _rhs,
                  self.data[2] * _rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.data[0] / _rhs,
                  self.data[1] / _rhs,
                  self.data[2] / _rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _rhs: f32) {
        self.data[0] /= _rhs;
        self.data[1] /= _rhs;
        self.data[2] /= _rhs;
    }
}
