mod vector;
mod linalg;
mod stat;

pub use math::vector::{Vec3, DotProduct};
pub use math::linalg::{normalize, lerp, slerp, distance};
pub use math::stat::{variance, into_variance};
