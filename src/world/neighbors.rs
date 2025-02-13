use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ProvinceNeighbors{
    pub prov_neighbors: Vec<Entity>
}

impl ProvinceNeighbors{
    pub fn new() -> Self {
        ProvinceNeighbors{prov_neighbors: Vec::new()}
    }
}