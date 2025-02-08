use crate::map_gen::city::City;
use bevy::prelude::*;


/// A province (smallest possible geographic subdivision) within the map.
pub struct Province {
    /// The primary city for this province.
    pub city: City,

    /// The province borders
    pub borders: Vec<(Vec2, Vec2)>,

    /// The connection edges between provinces
    pub connected_edges: Vec<(Vec2, Vec2)>,
}

impl Province {
    pub fn new(city: City) -> Self {
        Province{
            city,
            borders: vec![],
            connected_edges: vec![],
        }
    }
}