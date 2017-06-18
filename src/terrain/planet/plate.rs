use math::Vec3;
use math::normalize;

use std::f32;

use rand::thread_rng;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Clone)]
pub struct Plate {
    pub id: u32,
    pub tiles: Vec<u32>,
    pub base_elevation: f32,
    pub axis_of_rotation: Vec3<f32>,
    pub angular_velocity: f32,
}

impl Plate {
    pub fn new(id: u32) -> Plate {
        let mut rng = thread_rng();
        let ocean_ratio = 0.6;
        let base_elevation = if rng.next_f32() < ocean_ratio {
            let between = Range::new(-500.0, -100.0);
            between.ind_sample(&mut rng)
        } else {
            let between = Range::new(-50.0, 250.0);
            between.ind_sample(&mut rng)
        };
        let between = Range::new(-1.0, 1.0);
        let mut axis = Vec3::origo();
        while axis.length() < 0.01 {
            axis.x = between.ind_sample(&mut rng);
            axis.y = between.ind_sample(&mut rng);
            axis.z = between.ind_sample(&mut rng);
        }
        axis = normalize(axis);
        let rotation_speed = Range::new(0.1, 0.4).ind_sample(&mut rng);
        Plate {
            id: id,
            tiles: Vec::new(),
            base_elevation: base_elevation,
            axis_of_rotation: axis,
            angular_velocity: rotation_speed,
        }
    }

    pub fn add_tile(&mut self, tile_idx: u32) {
        self.tiles.push(tile_idx);
    }
}
