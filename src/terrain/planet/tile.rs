use math::Vec3;

use terrain::planet::{VertexIndex, BorderIndex};

use std::f32;
use std::slice::Iter;
use std::iter::Zip;

#[derive(Clone, Debug)]
pub struct Tile {
    vertices: Vec<VertexIndex>,
    num_vertices: usize,
    pub midpoint: VertexIndex,
    pub borders: Vec<BorderIndex>,
    pub plate_id: u32,
    pub movement_vector: Vec3<f32>,
}

impl Tile {
    pub fn new(mut vertices: Vec<VertexIndex>, midpoint: VertexIndex) -> Tile {
        let num_vertices = vertices.len();
        let first = vertices[0];
        vertices.push(first);
        Tile {
            vertices: vertices,
            num_vertices: num_vertices,
            midpoint: midpoint,
            borders: Vec::new(),
            plate_id: 0,
            movement_vector: Vec3::origo(),
        }
    }

    pub fn vertices_iter(&self) -> Iter<VertexIndex> {
        self.vertices[0..self.num_vertices].iter()
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn vertex_pairs(&self) -> Zip<Iter<u32>, Iter<u32>> {
        let p1 = self.vertices[0..self.num_vertices].iter();
        let p2 = self.vertices[1..].iter();
        p1.zip(p2)
    }

    fn index_of(&self, a: VertexIndex) -> Option<usize> {
        self.vertices.iter().position(|x| *x == a)
    }

    pub fn has_edge(&self, a: VertexIndex, b: VertexIndex) -> bool {
        if let Some(idx) = self.index_of(a) {
            let n = self.num_vertices;
            let before = (idx + n - 1) % n;
            let after = (idx + 1) % n;
            self.vertices[before] == b || self.vertices[after] == b
        } else {
            false
        }
    }
}
