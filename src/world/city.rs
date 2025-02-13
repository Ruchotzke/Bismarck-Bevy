use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct City {
    pub pos: Vec2,
}

impl City {
    pub fn new(x: f32, y: f32) -> Self {
        City { pos: Vec2::new(x, y) }
    }
}