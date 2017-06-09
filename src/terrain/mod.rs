mod edge;
mod face;
mod node;
mod generator;

use math::Vec3;

pub use terrain::edge::Edge;
pub use terrain::face::Face;
pub use terrain::node::Node;
pub use terrain::generator::Terrain;

pub type Vertex = Vec3<f32>;
pub type Index3 = Vec3<u32>;
