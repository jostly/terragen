use math::Vec3;
use math::normalize;

use std::f32;
use std::collections::HashSet;

use rand::thread_rng;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use super::{BorderIndex, PlateIndex, TileIndex};

#[derive(Debug, Clone)]
pub struct Plate {
    pub id: PlateIndex,
    pub tiles: Vec<TileIndex>,
    pub borders: HashSet<BorderIndex>,
    pub base_elevation: f32,
    pub axis_of_rotation: Vec3<f32>,
    pub angular_velocity: f32,
}

impl Plate {
    pub fn new(id: PlateIndex) -> Plate {
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
            borders: HashSet::new(),
            base_elevation: base_elevation,
            axis_of_rotation: axis,
            angular_velocity: rotation_speed,
        }
    }

    pub fn add_tile(&mut self, tile_idx: TileIndex, tile_borders: &[BorderIndex]) {
        self.tiles.push(tile_idx);
        let tile_borders = tile_borders.iter().cloned().collect();
        self.borders = self.borders.symmetric_difference(&tile_borders).cloned().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_plate() {
        let plate = Plate::new(1);
        assert_eq!(plate.id, 1);
        assert_eq!(plate.tiles, Vec::new());
        assert_eq!(plate.borders, HashSet::new());
    }

    #[test]
    fn plate_with_one_tile() {
        let mut plate = Plate::new(1);
        let border = vec!(2, 3, 5, 8, 13);
        plate.add_tile(17, &border);
        assert_eq!(plate.tiles, vec!(17));
        assert_eq!(plate.borders, border.iter().cloned().collect());
    }

    #[test]
    fn plate_with_two_tiles() {
        let mut plate = Plate::new(1);
        let border_1 = vec!(2, 3, 5, 8, 13);
        let border_2 = vec!(12, 13, 14, 15, 16, 17);
        plate.add_tile(1, &border_1);
        plate.add_tile(2, &border_2);
        let expected_border = vec!(2, 3, 5, 8, 12, 14, 15, 16, 17).iter().cloned().collect();

        assert_eq!(plate.tiles, vec!(1, 2));
        assert_eq!(plate.borders, expected_border);
    }
}
