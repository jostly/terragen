#[derive(Clone, Debug)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
    pub faces: Vec<u32>,
}

impl Edge {
    pub fn new(a: u32, b: u32) -> Edge {
        if a <= b {
            Edge {
                a: a,
                b: b,
                faces: Vec::new(),
            }
        } else {
            Edge::new(b, a)
        }
    }

    pub fn add_face(&mut self, face_index: u32) -> &Self {
        self.faces.push(face_index);
        self
    }
}
