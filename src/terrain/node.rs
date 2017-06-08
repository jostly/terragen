use terrain::Vertex;

#[derive(Clone)]
pub struct Node {
    pub point: Vertex,
    pub faces: Vec<u32>,
    pub edges: Vec<u32>,
    pub elevation: f32,
}

impl Node {
    pub fn new(point: Vertex, elevation: f32) -> Node {
        Node {
            point: point,
            faces: Vec::new(),
            edges: Vec::new(),
            elevation: elevation,
        }
    }

    pub fn add_edge(&mut self, edge_index: u32) -> &Self {
        self.edges.push(edge_index);
        self
    }
}
