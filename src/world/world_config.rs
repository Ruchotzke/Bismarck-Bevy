use std::ops::Range;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct WorldConfig {
    pub world_area: [Range<f64>; 2],
    pub city_pos_radius: f64
}

impl WorldConfig {
    pub fn new() -> Self {
        WorldConfig {
            world_area: [-600.0..600.0, -350.0..350.0],
            city_pos_radius: 30.0,
        }
    }
}