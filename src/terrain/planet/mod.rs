use math::Vec3;

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;

pub struct Tile {
    pub border: Vec<VertexIndex>,
    pub midpoint: VertexIndex,
}

impl Tile {
    pub fn new(border: Vec<VertexIndex>, midpoint: VertexIndex) -> Tile {
        Tile {
            border: border,
            midpoint: midpoint,
        }
    }
}

pub struct Planet {
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
}

impl Planet {
    pub fn new(vertices: Vec<Vertex>, tiles: Vec<Tile>) -> Planet {
        Planet {
            vertices: vertices,
            tiles: tiles,
        }
    }
}
