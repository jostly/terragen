mod vector;
mod linalg;
mod stat;

pub use math::vector::{Vec3, DotProduct};
pub use math::linalg::{normalize, lerp, slerp, distance};
pub use math::stat::{variance, into_variance};

pub fn sorted_pair<T>(a: T, b: T) -> (T, T)
    where T: Ord
{
    if a <= b { (a, b) } else { (b, a) }
}
