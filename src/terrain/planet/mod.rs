use math::Vec3;
use math::normalize;

use std::f32;

use rand::thread_rng;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;
pub type TileIndex = u32;

#[derive(Clone, Debug)]
pub struct Tile {
    pub border: Vec<VertexIndex>,
    pub midpoint: VertexIndex,
    pub elevation: f32,
    pub plate_id: u32,
}

impl Tile {
    pub fn new(border: Vec<VertexIndex>, midpoint: VertexIndex, elevation: f32) -> Tile {
        Tile {
            border: border,
            midpoint: midpoint,
            elevation: elevation,
            plate_id: 0,
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
    pub fn new(id: u32) -> Plate {
        Plate {
            id: id,
            tiles: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn center(&self, planet: &Planet) -> Vertex {
        let mut midpoint = Vec3::origo();
        if !self.tiles.is_empty() {
            for tile_idx in self.tiles.iter() {
                let tile = &planet.tiles[*tile_idx as usize];
                let point = &planet.vertices[tile.midpoint as usize];
                midpoint += point;
            }
            normalize(midpoint)
        } else {
            midpoint
        }
    }

    pub fn add_tile(&mut self, tile_idx: TileIndex) {
        self.tiles.push(tile_idx);
    }
}

pub struct Planet {
    pub vertices: Vec<Vertex>,
    pub tiles: Vec<Tile>,
    pub vertex_to_tiles: Vec<Vec<TileIndex>>,
    pub tile_neighbours: Vec<Vec<TileIndex>>,
    pub num_corners: usize,
    pub num_tiles: usize,
    pub num_plates: usize,
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
        Planet {
            vertices: vertices,
            tiles: tiles,
            vertex_to_tiles: vertex_tiles,
            tile_neighbours: tile_neighbours,
            num_corners: num_corners,
            num_tiles: num_tiles,
            num_plates: 0,
            plates: Vec::new(),
        }
    }

    #[allow(dead_code)]
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

    fn initialize_plates(&mut self, num_plates: usize) -> Vec<(TileIndex, u32)> {
        // Clear previous plate assignments
        for tile in self.tiles.iter_mut() {
            tile.plate_id = 0;
        }

        let mut plates: Vec<Plate> = Vec::new();
        let mut rng = thread_rng();
        let between = Range::new(0, self.num_corners);

        let mut failed_count = 0;

        let mut assign_queue = Vec::new();

        while plates.len() < num_plates && failed_count < 10000 {
            let corner = &self.vertex_to_tiles[between.ind_sample(&mut rng)];
            let mut adjacent_to_existing_plate = false;
            for tile_idx in corner.iter() {
                if self.tiles[*tile_idx as usize].plate_id > 0 {
                    adjacent_to_existing_plate = true;
                    failed_count += 1;
                    break;
                }
            }
            if adjacent_to_existing_plate {
                continue;
            }

            failed_count = 0;

            let mut plate = Plate::new(1 + plates.len() as u32);

            for tile_idx in corner.iter() {
                plate.add_tile(*tile_idx);
                self.tiles[*tile_idx as usize].plate_id = plate.id;
            }

            for tile_idx in corner.iter() {
                for other_tile_idx in self.tile_neighbours[*tile_idx as usize].iter() {
                    let other_tile = &self.tiles[*other_tile_idx as usize];
                    if other_tile.plate_id == 0 {
                        assign_queue.push((*other_tile_idx, plate.id));
                    }
                }
            }

            plates.push(plate);
        }

        self.num_plates = plates.len();
        self.plates = plates;

        assign_queue
    }

    pub fn assign_plates(&mut self) -> &Self {

        for plate in self.plates.iter() {
            for tile_idx in plate.tiles.iter() {
                self.tiles[*tile_idx as usize].plate_id = plate.id;
            }
        }
        self
    }

    pub fn grow_plates(&mut self) {
        let mut assign_queue = self.initialize_plates(15);

        while !assign_queue.is_empty() {
            let mut rng = thread_rng();
            let idx = (rng.next_f32().powf(2.0) * assign_queue.len() as f32).floor() as usize;
            let (tile_idx, plate_id) = assign_queue.remove(idx);

            if self.tiles[tile_idx as usize].plate_id == 0 {
                self.tiles[tile_idx as usize].plate_id = plate_id;
                self.plates[plate_id as usize - 1].add_tile(tile_idx);
                for other_idx in self.tile_neighbours[tile_idx as usize].iter() {
                    let other_tile = &self.tiles[*other_idx as usize];
                    if other_tile.plate_id == 0 {
                        assign_queue.push((*other_idx, plate_id));
                    }
                }
            }

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

        self.assign_plates();
    }
}
