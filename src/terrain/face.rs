use terrain::{Index3, Edge};

#[derive(Clone, Debug)]
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

    pub fn node(&self, i: u32) -> u32 {
        match i % 3 {
            0 => self.points.x,
            1 => self.points.y,
            _ => self.points.z,
        }
    }

    pub fn edge(&self, i: u32) -> u32 {
        match i % 3 {
            0 => self.edges.x,
            1 => self.edges.y,
            _ => self.edges.z,
        }
    }

    #[inline]
    pub fn opposite_node_index_of_edge(&self, edge: &Edge) -> u32 {
        self.opposite_node_index(edge.a, edge.b)
    }

    pub fn opposite_node_index(&self, a: u32, b: u32) -> u32 {
        if self.points.x == a && self.points.y == b {
            2
        } else if self.points.x == b && self.points.y == a {
            2
        } else if self.points.y == a && self.points.z == b {
            0
        } else if self.points.y == b && self.points.z == a {
            0
        } else if self.points.x == a && self.points.z == b {
            1
        } else if self.points.x == b && self.points.z == a {
            1
        } else {
            panic!("Face {:?} did not contain nodes {} and {}", self, a, b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_node_by_index() {
        let face = Face::new(Index3::new(4, 7, 11), Index3::new(23, 34, 68));

        assert_eq!(face.node(0), 4, "Index 0");
        assert_eq!(face.node(1), 7, "Index 1");
        assert_eq!(face.node(2), 11, "Index 2");
        assert_eq!(face.node(3), 4, "Index 3 wraps around to index 0");
    }

    #[test]
    fn get_edge_by_index() {
        let face = Face::new(Index3::new(4, 7, 11), Index3::new(23, 34, 68));

        assert_eq!(face.edge(0), 23, "Index 0");
        assert_eq!(face.edge(1), 34, "Index 1");
        assert_eq!(face.edge(2), 68, "Index 2");
        assert_eq!(face.edge(3), 23, "Index 3 wraps around to index 0");
    }

    #[test]
    fn get_opposide_node_index() {
        let face = Face::new(Index3::new(4, 7, 11), Index3::new(23, 34, 68));

        assert_eq!(face.opposite_node_index(4, 7), 2);
        assert_eq!(face.opposite_node_index(7, 4), 2);

        assert_eq!(face.opposite_node_index(11, 7), 0);
        assert_eq!(face.opposite_node_index(7, 11), 0);

        assert_eq!(face.opposite_node_index(4, 11), 1);
        assert_eq!(face.opposite_node_index(11, 4), 1);
    }
}
