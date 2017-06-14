use math::Vec3;
use math::DotProduct;
use math::normalize;

use std::f32;

use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;
pub type TileIndex = u32;

#[derive(Clone, Debug)]
pub struct Tile {
    pub border: Vec<VertexIndex>,
    pub midpoint: VertexIndex,
    pub elevation: f32,
    pub group_id: u32,
}

impl Tile {
    pub fn new(border: Vec<VertexIndex>, midpoint: VertexIndex, elevation: f32) -> Tile {
        Tile {
            border: border,
            midpoint: midpoint,
            elevation: elevation,
            group_id: 0,
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

#[derive(Debug, Clone)]
struct Plate {
    id: u32,
    tiles: Vec<TileIndex>,
}

impl Plate {
    pub fn new(id: u32, root: TileIndex) -> Plate {
        Plate {
            id: id,
            tiles: vec![root],
        }
    }

    pub fn center(&self, planet: &Planet) -> Vertex {
        let mut midpoint = Vec3::origo();
        for tile_idx in self.tiles.iter() {
            let tile = &planet.tiles[*tile_idx as usize];
            let point = &planet.vertices[tile.midpoint as usize];
            midpoint += point;
        }
        normalize(midpoint)
    }

    pub fn root(&self) -> TileIndex {
        self.tiles[0]
    }

    pub fn add_tile(&mut self, tile_idx: TileIndex) {
        self.tiles.push(tile_idx);
    }
}

struct Groups {
    group_centers: Vec<usize>,
    unassigned_indexes: Vec<usize>,
}

pub struct Planet {
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
    pub vertex_to_tiles: Vec<Vec<TileIndex>>,
    pub tile_neighbours: Vec<Vec<TileIndex>>,
    pub num_tiles: usize,
    pub num_groups: usize,
    plates: Vec<Plate>,
}

impl Planet {
    pub fn new(vertices: Vec<Vertex>, borders: Vec<Vec<VertexIndex>>) -> Planet {
        let num_tiles = borders.len();
        let num_corners = vertices.len() - num_tiles;

        let mut tiles = Vec::with_capacity(borders.len());
        let mut i = num_corners;
        for b in borders.into_iter() {
            let tile = Tile::new(b, i as u32, 1.0);
            tiles.push(tile);
            i += 1;
        }

        let mut vertex_tiles = vec![vec!(); num_corners];

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
        let plates = Self::initialize_plates(num_tiles);
        Planet {
            vertices: vertices,
            tiles: tiles,
            vertex_to_tiles: vertex_tiles,
            tile_neighbours: tile_neighbours,
            num_tiles: num_tiles,
            num_groups: plates.len(),
            plates: plates,
        }
    }

    pub fn normalize_elevations(mut self) -> Self {
        let mut min_elevation = f32::MAX;
        let mut max_elevation = f32::MIN;

        for tile in self.tiles.iter() {
            let e = tile.elevation;
            if e < min_elevation {
                min_elevation = e;
            }
            if e > max_elevation {
                max_elevation = e;
            }
        }

        let scale = max_elevation - min_elevation;

        if scale.abs() > 0.0 {
            for tile in self.tiles.iter_mut() {
                tile.elevation = (tile.elevation - min_elevation) / scale;
            }
        }

        self
    }

    fn initialize_plates(num_tiles: usize) -> Vec<Plate> {
        let mut plates: Vec<Plate> = Vec::new();

        let mut rng = thread_rng();

        let max_range = num_tiles as u32;
        let between = Range::new(max_range / 32 + 1, max_range / 24 + 2);

        let num_plates = between.ind_sample(&mut rng);

        let between = Range::new(0, num_tiles);

        for plate_id in 0..num_plates {
            // assign to random tiles
            loop {
                let tile_idx = between.ind_sample(&mut rng) as TileIndex;
                if !plates.iter().any(|p| p.root() == tile_idx) {
                    plates.push(Plate::new(plate_id, tile_idx));
                    break;
                }
            }
        }

        println!("Generated {} plates", plates.len());

        plates
    }

    pub fn assign_groups(&mut self) -> &Self {
        // Clear previous group assignments
        for tile in self.tiles.iter_mut() {
            tile.group_id = 0;
        }

        for (idx, plate) in self.plates.iter().enumerate() {
            for tile_idx in plate.tiles.iter() {
                self.tiles[*tile_idx as usize].group_id = (plate.id + 1) as u32;
            }
        }

        self
    }

    pub fn grow_groups(&mut self) {
        let mut unassigned_indexes = Vec::new();
        for (idx, tile) in self.tiles.iter().enumerate() {
            if tile.group_id == 0 {
                unassigned_indexes.push(idx);
            }
        }

        while unassigned_indexes.len() > 0 {
            debug!("Assigning, {} to go...", unassigned_indexes.len());
            let mut nearest_index = 0;
            let mut nearest_group = 0;
            let mut nearest_distance = f32::MAX;
            for (idx, tile_idx) in unassigned_indexes.iter().enumerate() {
                let tile = &self.tiles[*tile_idx];
                let neighbours = &self.tile_neighbours[*tile_idx];
                for ne in neighbours.iter() {
                    let other_tile = &self.tiles[*ne as usize];
                    if other_tile.group_id != 0 {
                        let midpoint = &self.vertices[tile.midpoint as usize];
                        let plate = &self.plates[(other_tile.group_id - 1) as usize];
                        let other_midpoint = plate.center(self);

                        let distance = (other_midpoint.dot(midpoint) /
                                        (other_midpoint.length() * midpoint.length()))
                                .acos();

                        //                        let distance = (other_midpoint - midpoint).length_squared();
                        if distance < nearest_distance {
                            nearest_group = other_tile.group_id;
                            nearest_index = idx;
                            nearest_distance = distance;
                            debug!("Distance from {:?} to {:?} is {}",
                                   tile,
                                   other_tile,
                                   distance);
                        }
                    }
                }
            }

            let tile_idx = unassigned_indexes.swap_remove(nearest_index);
            self.tiles[tile_idx].group_id = nearest_group;
            self.plates[(nearest_group - 1) as usize].add_tile(tile_idx as u32);
            debug!("Assigned {:?}", self.tiles[tile_idx]);
            //break;
        }
    }

    pub fn merge_plates(&mut self) {
        let min_plate_size = self.num_tiles / 30;
        println!("Minimum plate size is {}", min_plate_size);
        let mut plates = self.plates.clone();

        loop {
            plates.sort_by(|a, b| a.tiles.len().cmp(&b.tiles.len()));
            if plates[0].tiles.len() >= min_plate_size {
                println!("All plates big enough");
                break;
            }

            // Find neighbouring plates

            let mut neighbouring_plates = Vec::new();
            {

                let plate = &plates[0];
                'lp: for i in 1..plates.len() {
                    let other_plate = &plates[i];

                    for own_tile in plate.tiles.iter() {
                        for other_tile in self.tile_neighbours[*own_tile as usize].iter() {
                            if other_plate.tiles.contains(&other_tile) {
                                neighbouring_plates.push(i);
                                break 'lp;
                            }
                        }

                    }
                }
            }

            // merge with neighbouring plate
            let other_plate_idx = neighbouring_plates[0];
            println!("Merging plate {} size {} with plate {} size {}",
                     plates[0].id,
                     plates[0].tiles.len(),
                     plates[other_plate_idx].id,
                     plates[other_plate_idx].tiles.len());
            let mut tiles = plates[0].tiles.clone();
            plates[other_plate_idx].tiles.append(&mut tiles);
            plates.swap_remove(0);
            //break;
        }

        self.plates = plates;

        self.assign_groups();

    }
}
