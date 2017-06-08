use terrain::Index3;

#[derive(Clone)]
pub struct Face {
    pub points: Index3,
    pub edges: Index3,
}

impl Face {
    pub fn new(points: Index3, edges: Index3) -> Face {
        Face {
            points: points,
            edges: edges,
        }
    }
}
