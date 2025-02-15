use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Pop{
    /// The number of people in this population.
    pub size: u32,

    /// The home city of this population
    /// (BEVY: CITY COMPONENT)
    pub home: Entity,
}