use math::Vec3;

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;

pub struct Tile {
    pub border: Vec<VertexIndex>,
    pub midpoint: VertexIndex,
    pub elevation: f32,
}

impl Tile {
    pub fn new(border: Vec<VertexIndex>, midpoint: VertexIndex, elevation: f32) -> Tile {
        Tile {
            border: border,
            midpoint: midpoint,
            elevation: elevation,
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