mod edge;
mod face;
mod node;
mod generator;

use math::*;
pub use terrain::edge::Edge;
pub use terrain::face::Face;
pub use terrain::node::Node;
pub use terrain::generator::Terrain;

use rand::random;
use std::f32;
use std::collections::HashMap;

pub type Vertex = Vec3<f32>;
pub type Index3 = Vec3<u32>;
