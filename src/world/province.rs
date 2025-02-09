use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Province {
    /// The central city for this province
    pub city: Vec2,

    /// The borders in 2D space for this province
    pub borders: Vec<(Vec2, Vec2)>,

    /// The set of neighboring provinces to this one
    pub neighboring_provinces: Vec<Entity>,
}

impl Province {
    pub fn new() -> Self {
        Province {
            city: Vec2::ZERO,
            borders: Vec::new(),
            neighboring_provinces: Vec::new(),
        }
    }
}