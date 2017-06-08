use std::ops::*;

#[derive(Clone, Debug, PartialEq)]
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

macro_rules! componentwise_binop {
    ($Trait: ident, $method: ident) => {
        impl<T> $Trait for Vec3<T>
            where T: $Trait
        {
            type Output = Vec3<<T as $Trait>::Output>;

            #[inline]
            fn $method(self, other: Vec3<T>) -> Self::Output {
                Vec3::new(
                    self.x.$method(other.x),
                    self.y.$method(other.y),
                    self.z.$method(other.z)
                    )
            }
        }

        impl<'a, T> $Trait<&'a Vec3<T>> for Vec3<T>
            where T: $Trait<&'a T>
        {
            type Output = Vec3<<T as $Trait<&'a T>>::Output>;

            #[inline]
            fn $method(self, other: &'a Vec3<T>) -> Self::Output {
                Vec3::new(
                    self.x.$method(&other.x),
                    self.y.$method(&other.y),
                    self.z.$method(&other.z)
                    )
            }
        }

        impl<'b, T> $Trait<Vec3<T>> for &'b Vec3<T>
            where T: $Trait + Clone
        {
            type Output = Vec3<<T as $Trait>::Output>;

            #[inline]
            fn $method(self, other: Vec3<T>) -> Self::Output {
                let x = self.x.clone();
                let y = self.y.clone();
                let z = self.z.clone();
                Vec3::new(
                    x.$method(other.x),
                    y.$method(other.y),
                    z.$method(other.z)
                    )
            }
        }

        impl<'a, 'b, T> $Trait<&'a Vec3<T>> for &'b Vec3<T>
            where T: $Trait<&'a T> + Clone
        {
            type Output = Vec3<<T as $Trait<&'a T>>::Output>;

            #[inline]
            fn $method(self, other: &'a Vec3<T>) -> Self::Output {
                let x = self.x.clone();
                let y = self.y.clone();
                let z = self.z.clone();
                Vec3::new(
                    x.$method(&other.x),
                    y.$method(&other.y),
                    z.$method(&other.z)
                    )
            }
        }
    }
}

macro_rules! componentwise_assignop {
    ($Trait: ident, $method: ident) => {
        impl<T> $Trait for Vec3<T>
            where T: $Trait
        {
            fn $method(&mut self, other: Vec3<T>) {
                self.x.$method(other.x);
                self.y.$method(other.y);
                self.z.$method(other.z);
            }
        }

        impl<'a, T> $Trait<&'a Vec3<T>> for Vec3<T>
            where T: $Trait + Copy
        {
            fn $method(&mut self, other: &'a Vec3<T>) {
                self.x.$method(other.x);
                self.y.$method(other.y);
                self.z.$method(other.z);
            }
        }
    }
}

macro_rules! scalar_binop {
    ($Trait: ident, $method: ident) => {
        impl<T, S> $Trait<S> for Vec3<T>
            where T: $Trait<S>,
                  S: Copy
        {
            type Output = Vec3<<T as $Trait<S>>::Output>;

            fn $method(self, _rhs: S) -> Self::Output {
                Vec3::new(self.x.$method(_rhs),
                          self.y.$method(_rhs),
                          self.z.$method(_rhs))
            }
        }

        impl<'a, T, S> $Trait<S> for &'a Vec3<T>
            where T: $Trait<S> + Copy,
                  S: Copy
        {
            type Output = Vec3<<T as $Trait<S>>::Output>;

            fn $method(self, _rhs: S) -> Self::Output {
                Vec3::new(self.x.$method(_rhs),
                          self.y.$method(_rhs),
                          self.z.$method(_rhs))
            }
        }
    }
}

macro_rules! scalar_assignop {
    ($Trait: ident, $method: ident) => {
        impl<T, S> $Trait<S> for Vec3<T>
            where T: $Trait<S>,
                  S: Copy
        {
            fn $method(&mut self, _rhs: S) {
                self.x.$method(_rhs);
                self.y.$method(_rhs);
                self.z.$method(_rhs);
            }
        }
    }
}

componentwise_binop!(Add, add);
componentwise_assignop!(AddAssign, add_assign);
componentwise_binop!(Sub, sub);
componentwise_assignop!(SubAssign, sub_assign);

scalar_binop!(Mul, mul);
scalar_assignop!(MulAssign, mul_assign);

scalar_binop!(Div, div);
scalar_assignop!(DivAssign, div_assign);

pub trait DotProduct<T> {
    type Output;

    fn dot(&self, other: T) -> Self::Output;
}

impl<'a, T> DotProduct<&'a Vec3<T>> for Vec3<T>
    where T: Mul<T, Output = T> + Add<T, Output = T> + Copy
{
    type Output = T;

    fn dot(&self, other: &'a Vec3<T>) -> Self::Output {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl<T> DotProduct<Vec3<T>> for Vec3<T>
    where T: Mul<T, Output = T> + Add<T, Output = T> + Copy
{
    type Output = T;

    fn dot(&self, other: Vec3<T>) -> Self::Output {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

//impl<'a, T> DotProduct<Vec3<T>> for &'a Vec3<T>
//where T: Mul<T, Output = T> + Add<T, Output = T> + Copy
//{
//type Output = T;

//fn dot(&self, other: Vec3<T>) -> Self::Output {
//(self.x * other.x) + (self.y * other.y) + (self.z * other.z)
//}
//}

//impl<'a, 'b, T> DotProduct<&'b Vec3<T>> for &'a Vec3<T>
//where T: Mul<T, Output = T> + Add<T, Output = T> + Copy
//{
//type Output = T;

//fn dot(&self, other: &'b Vec3<T>) -> Self::Output {
//(self.x * other.x) + (self.y * other.y) + (self.z * other.z)
//}
//}

impl Vec3<f32> {
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! binop_test {
        ($func: ident; $a: expr, $b: expr => $method: ident => $expected: expr) => {
            #[test]
            fn $func() {
                let ref ra = $a;
                let ref rb = $b;

                assert_eq!(ra.$method(rb), $expected, "&Vec3, &Vec3");
                assert_eq!(ra.$method($b), $expected, "&Vec3, Vec3");
                assert_eq!($a.$method(rb), $expected, "Vec3, &Vec3");
                assert_eq!($a.$method($b), $expected, "Vec3, Vec3");
            }
        }
    }

    macro_rules! assignop_test {
        ($func: ident; $a: expr, $b: expr => $method: ident => $expected: expr) => {
            #[test]
            fn $func() {
                let mut a = $a;
                let ref rb = $b;

                a.$method($b);
                assert_eq!(a, $expected, "Vec3, Vec3");

                a = $a;
                a.$method(rb);
                assert_eq!(a, $expected, "Vec3, &Vec3");
            }
        }
    }

    macro_rules! assignop_noref_test {
        ($func: ident; $a: expr, $b: expr => $method: ident => $expected: expr) => {
            #[test]
            fn $func() {
                let mut a = $a;
                let b = $b;
                let expected = $expected;

                a.$method(b);
                assert_eq!(a, expected, "Vec3, Vec3");
            }
        }
    }

    binop_test!(add_vectors; Vec3::new(1, 2, 3), Vec3::new(4, 7, 11) => add => Vec3::new(5, 9, 14));
    binop_test!(sub_vectors; Vec3::new(4, 7, 11), Vec3::new(1, 2, 3) => sub => Vec3::new(3, 5, 8));
    assignop_test!(add_assign_vectors; Vec3::new(1, 2, 3), Vec3::new(4, 7, 11) => add_assign => Vec3::new(5, 9, 14));
    assignop_test!(sub_assign_vectors; Vec3::new(4, 7, 11), Vec3::new(1, 2, 3) => sub_assign => Vec3::new(3, 5, 8));

    binop_test!(scalar_mul; Vec3::new(4, 7, 11), 2 => mul => Vec3::new(8, 14, 22));
    assignop_noref_test!(scalar_mul_assign; Vec3::new(4, 7, 11), 2 => mul_assign => Vec3::new(8, 14, 22));

    binop_test!(scalar_div; Vec3::new(8, 14, 22), 2 => div => Vec3::new(4, 7, 11));
    assignop_noref_test!(scalar_div_assign; Vec3::new(8, 14, 22), 2 => div_assign => Vec3::new(4, 7, 11));

    binop_test!(dot_product; Vec3::new(3, 4, 5), Vec3::new(4, 7, 11) => dot => 3 * 4 + 4 * 7 + 5 * 11);

}
