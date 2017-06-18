use math::sorted_pair;

use terrain::planet::{VertexIndex, TileIndex};

#[derive(Clone, Debug)]
pub struct Border {
    pub vertices: (VertexIndex, VertexIndex),
    pub tiles: (TileIndex, TileIndex),
}

impl Border {
    pub fn new(va: VertexIndex, vb: VertexIndex, ta: TileIndex, tb: TileIndex) -> Border {
        Border {
            vertices: sorted_pair(va, vb),
            tiles: sorted_pair(ta, tb),
        }
    }

    pub fn other_tile(&self, tile_index: TileIndex) -> Option<TileIndex> {
        if self.tiles.0 == tile_index {
            Some(self.tiles.1)
        } else if self.tiles.1 == tile_index {
            Some(self.tiles.0)
        } else {
            None
        }
    }
}
