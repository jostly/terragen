use math::Vec3;

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;
pub type TileIndex = u32;

#[derive(Clone, Debug)]
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

    fn index_of(&self, a: VertexIndex) -> Option<usize> {
        self.border.iter().position(|x| *x == a)
    }

    pub fn has_edge(&self, a: VertexIndex, b: VertexIndex) -> bool {
        if let Some(idx) = self.index_of(a) {
            let n = self.border.len();
            let before = (idx + n - 1) % n;
            let after = (idx + 1) % n;
            self.border[before] == b || self.border[after] == b
        } else {
            false
        }
    }
}

pub struct Planet {
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
    pub vertex_to_tiles: Vec<Vec<TileIndex>>,
    pub tile_neighbours: Vec<Vec<TileIndex>>,
}

impl Planet {
    pub fn new(vertices: Vec<Vertex>, tiles: Vec<Tile>) -> Planet {
        let mut vertex_tiles = vec![vec!(); vertices.len() - tiles.len()]; // Subtract midpoints

        for (idx, tile) in tiles.iter().enumerate() {
            for vi in tile.border.iter() {
                vertex_tiles[*vi as usize].push(idx as TileIndex);
            }
        }

        let mut tile_neighbours = vec![vec!(); tiles.len()];

        for tile_idxs in vertex_tiles.iter() {
            for tidx in tile_idxs.iter() {
                let tile = &tiles[*tidx as usize];
                let mut a = tile.border[0];
                let n = tile.border.len();
                for j in 0..n {
                    let b = tile.border[(j + 1) % n];
                    if let Some(other) =
                        tile_idxs
                            .iter()
                            .find(|t| **t != *tidx && tiles[**t as usize].has_edge(a, b)) {
                        let tn = &mut tile_neighbours[*tidx as usize];
                        if !tn.contains(other) {
                            tn.push(*other);
                        }
                    }
                    a = b;
                }
            }
        }
        /*
        for (idx, tile_idxs) in vertex_tiles.iter().enumerate() {
            println!("Vertex {} -> {:?}", idx, tile_idxs);
        }

        for (idx, tile) in tiles.iter().enumerate() {
            let neighbours = &tile_neighbours[idx];
            println!("Tile {} ({:?}) -> {:?}", idx, tile, neighbours);

        }
*/
        Planet {
            vertices: vertices,
            tiles: tiles,
            vertex_to_tiles: vertex_tiles,
            tile_neighbours: tile_neighbours,
        }
    }
}
