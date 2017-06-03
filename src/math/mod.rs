use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
}

impl Vec3<f32> {
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normal(&self) -> Self {
        let l = self.length();
        if l == 0.0 { *self } else { *self / l }
    }
}

impl<T> Add for Vec3<T>
    where T: Add
{
    type Output = Vec3<<T as Add>::Output>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, T> Add<&'a Vec3<T>> for Vec3<T>
    where T: Add<&'a T>
{
    type Output = Vec3<<T as Add<&'a T>>::Output>;

    fn add(self, other: &'a Vec3<T>) -> Self::Output {
        Vec3::new(self.x + &other.x, self.y + &other.y, self.z + &other.z)
    }
}

impl<T> AddAssign for Vec3<T>
    where T: AddAssign
{
    fn add_assign(&mut self, other: Vec3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a, T> AddAssign<&'a Vec3<T>> for Vec3<T>
    where T: AddAssign<&'a T>
{
    fn add_assign(&mut self, other: &'a Vec3<T>) {
        self.x += &other.x;
        self.y += &other.y;
        self.z += &other.z;
    }
}

impl<T> Sub for Vec3<T>
    where T: Sub
{
    type Output = Vec3<<T as Sub>::Output>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T, M> Mul<M> for Vec3<T>
    where T: Mul<M>,
          M: Copy
{
    type Output = Vec3<<T as Mul<M>>::Output>;

    fn mul(self, _rhs: M) -> Self::Output {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl<T, D> Div<D> for Vec3<T>
    where T: Div<D>,
          D: Copy
{
    type Output = Vec3<<T as Div<D>>::Output>;

    fn div(self, _rhs: D) -> Self::Output {
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}

impl<T, D> DivAssign<D> for Vec3<T>
    where T: DivAssign<D>,
          D: Copy
{
    fn div_assign(&mut self, _rhs: D) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}
