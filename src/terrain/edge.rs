#[derive(Clone)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
}

impl Edge {
    pub fn new(a: u32, b: u32) -> Edge {
        if a <= b {
            Edge { a: a, b: b }
        } else {
            Edge { a: b, b: a }
        }
    }
}
