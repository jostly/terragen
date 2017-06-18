use math::Vec3;

use terrain::planet::{VertexIndex, BorderIndex};

use std::f32;

#[derive(Clone, Debug)]
pub struct Tile {
    pub vertices: Vec<VertexIndex>,
    pub midpoint: VertexIndex,
    pub borders: Vec<BorderIndex>,
    pub plate_id: u32,
    pub movement_vector: Vec3<f32>,
}

impl Tile {
    pub fn new(vertices: Vec<VertexIndex>, midpoint: VertexIndex) -> Tile {
        Tile {
            vertices: vertices,
            midpoint: midpoint,
            borders: Vec::new(),
            plate_id: 0,
            movement_vector: Vec3::origo(),
        }
    }

    fn index_of(&self, a: VertexIndex) -> Option<usize> {
        self.vertices.iter().position(|x| *x == a)
    }

    pub fn has_edge(&self, a: VertexIndex, b: VertexIndex) -> bool {
        if let Some(idx) = self.index_of(a) {
            let n = self.vertices.len();
            let before = (idx + n - 1) % n;
            let after = (idx + 1) % n;
            self.vertices[before] == b || self.vertices[after] == b
        } else {
            false
        }
    }
}
