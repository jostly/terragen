mod border;
mod plate;
mod tile;

use math::{Vec3, DotProduct};
use math::{normalize, sorted_pair};

use std::f32;
use std::collections::HashMap;
use std::slice::Iter;

use rand::thread_rng;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use noise::{NoiseModule, RidgedMulti};

pub use self::plate::Plate;
pub use self::border::Border;
pub use self::tile::Tile;

pub type Vertex = Vec3<f32>;
pub type VertexIndex = u32;
pub type TileIndex = u32;
pub type BorderIndex = u32;
pub type PlateIndex = u32;

pub struct Planet {
    vertices: Vec<Vertex>,
    tiles: Vec<Tile>,
    #[allow(dead_code)]
    borders: Vec<Border>,
    elevations: Vec<f32>,
    vertex_to_tiles: Vec<Vec<TileIndex>>,
    tile_neighbours: Vec<Vec<TileIndex>>,
    num_corners: usize,
    num_tiles: usize,
    num_plates: usize,
    plates: Vec<Plate>,
    scale: f32,
}

impl Planet {
    pub fn new(vertices: Vec<Vertex>, borders: Vec<Vec<VertexIndex>>) -> Planet {
        let num_tiles = borders.len();
        let num_corners = vertices.len() - num_tiles;
        let mut borders_map = HashMap::<(VertexIndex, VertexIndex), Vec<TileIndex>>::new();

        let mut tiles = Vec::with_capacity(borders.len());
        let mut i = num_corners;
        for b in borders.into_iter() {
            let tile = Tile::new(b, i as u32);
            tiles.push(tile);
            i += 1;
        }

        for (idx, tile) in tiles.iter().enumerate() {
            for (curr, prev) in tile.vertex_pairs() {
                let pair = sorted_pair(*curr, *prev);

                let mut tiles = borders_map.entry(pair).or_insert(Vec::new());
                tiles.push(idx as u32);
            }
        }

        let mut borders_vec = Vec::with_capacity(borders_map.len());
        for (v, t) in borders_map.iter() {
            let bix = borders_vec.len() as u32;
            assert!(t.len() == 2,
                    "Expected tiles for {:?} to have size 2, but was {:?}",
                    v,
                    t);
            let border = Border::new(v.0, v.1, t[0], t[1]);
            borders_vec.push(border);
            tiles[t[0] as usize].borders.push(bix);
            tiles[t[1] as usize].borders.push(bix);
        }
        drop(borders_map);

        let mut vertex_tiles = vec![vec!(); num_corners];

        for (idx, tile) in tiles.iter().enumerate() {
            for vi in tile.vertices_iter() {
                vertex_tiles[*vi as usize].push(idx as TileIndex);
            }
        }

        let mut tile_neighbours = vec![vec!(); tiles.len()];

        // Build tile neighbour map
        for (tidx, tile) in tiles.iter().enumerate() {
            for border_idx in tile.borders.iter() {
                let border = &borders_vec[*border_idx as usize];
                if let Some(other) = border.other_tile(tidx as u32) {
                    let tn = &mut tile_neighbours[tidx];
                    if !tn.contains(&other) {
                        tn.push(other);
                    }
                } else {
                    panic!("Tile #{} links to border #{}, but the latter does not link back",
                           tidx,
                           border_idx);
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

        let fbm = RidgedMulti::new();
        //fbm = fbm.set_frequency(7.5);

        let mut elevations = Vec::with_capacity(num_corners);
        for vert in vertices[0..num_corners].iter() {
            let v = [vert.x, vert.y, vert.z];
            let e = fbm.get(v) * 100.0;
            elevations.push(e);
        }

        let mut planet = Planet {
            vertices: vertices,
            tiles: tiles,
            borders: borders_vec,
            elevations: elevations,
            vertex_to_tiles: vertex_tiles,
            tile_neighbours: tile_neighbours,
            num_corners: num_corners,
            num_tiles: num_tiles,
            num_plates: 0,
            plates: Vec::new(),
            scale: 10.0,
        };

        planet.grow_plates();

        planet
    }

    pub fn tiles_iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }

    pub fn num_tiles(&self) -> usize {
        self.num_tiles
    }

    pub fn tile_normal(&self, tile: &Tile) -> Vertex {
        normalize(self.vertices[tile.midpoint as usize].clone())
    }

    pub fn tile_midpoint(&self, tile: &Tile) -> Vertex {
        &self.vertices[tile.midpoint as usize] * self.scale
    }

    pub fn tile_border_points(&self, tile: &Tile) -> Vec<Vertex> {
        tile.vertices_iter()
            .map(|vi| &self.vertices[*vi as usize] * self.scale)
            .collect()
    }

    pub fn tile_elevation(&self, tile: &Tile) -> f32 {
        let mut elevation = 0.0;
        let mut n = 0;
        for vi in tile.vertices_iter() {
            elevation += self.elevations[*vi as usize];
            n += 1;
        }
        elevation / n as f32 + self.plates[tile.plate_id as usize - 1].base_elevation
    }

    pub fn get_elevation_scale(&self) -> (f32, f32) {
        let mut min_elevation = f32::MAX;
        let mut max_elevation = f32::MIN;

        for tile in self.tiles.iter() {
            let e = self.tile_elevation(tile);
            if e < min_elevation {
                min_elevation = e;
            }
            if e > max_elevation {
                max_elevation = e;
            }
        }

        let scale = max_elevation - min_elevation;

        println!("Elevations: {} to {}", min_elevation, max_elevation);

        (min_elevation, scale)
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
            let corner = &self.vertex_to_tiles[between.ind_sample(&mut rng)].clone();
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
                plate.add_tile(*tile_idx, &self.tiles[*tile_idx as usize].borders);
                self.assign_plate_to_tile(&plate, *tile_idx);
                let tile = &mut self.tiles[*tile_idx as usize];
                tile.plate_id = plate.id;
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

    fn assign_plate_id_to_tile(&mut self, plate_id: u32, tile_index: u32) {
        let plate = &self.plates[plate_id as usize - 1];
        let movement_vector =
            self.calculate_movement_vector(plate, &self.tiles[tile_index as usize]);
        let tile = &mut self.tiles[tile_index as usize];
        tile.plate_id = plate_id;
        tile.movement_vector = movement_vector;
    }
    fn assign_plate_to_tile(&mut self, plate: &Plate, tile_index: u32) {
        let movement_vector =
            self.calculate_movement_vector(plate, &self.tiles[tile_index as usize]);
        let tile = &mut self.tiles[tile_index as usize];
        tile.plate_id = plate.id;
        tile.movement_vector = movement_vector;
    }
    fn calculate_movement_vector(&self, plate: &Plate, tile: &Tile) -> Vec3<f32> {
        let midpoint = &self.vertices[tile.midpoint as usize];
        let base_on_axis = &plate.axis_of_rotation * midpoint.dot(&plate.axis_of_rotation);
        let perpendicular = midpoint - base_on_axis;

        &plate.axis_of_rotation.cross(&perpendicular) * plate.angular_velocity
    }

    fn assign_plates(&mut self) -> &Self {

        for plate in self.plates.iter() {
            for tile_idx in plate.tiles.iter() {
                let tile = &mut self.tiles[*tile_idx as usize];
                tile.plate_id = plate.id;
            }
        }
        self
    }

    pub fn grow_plates(&mut self) {
        let mut assign_queue = self.initialize_plates(27);

        while !assign_queue.is_empty() {
            let mut rng = thread_rng();
            let idx = (rng.next_f32().powf(2.0) * assign_queue.len() as f32).floor() as usize;
            let (tile_idx, plate_id) = assign_queue.remove(idx);

            if self.tiles[tile_idx as usize].plate_id == 0 {
                self.assign_plate_id_to_tile(plate_id, tile_idx);
                self.plates[plate_id as usize - 1]
                    .add_tile(tile_idx, &self.tiles[tile_idx as usize].borders);
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
        let min_plate_size = self.num_tiles() / 30;
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
