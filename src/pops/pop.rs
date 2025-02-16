use std::ptr::null;
use bevy::prelude::*;

#[derive(Component, Copy)]
pub struct Pop{
    /// The number of people in this population.
    pub size: u32,

    /// The home city of this population
    /// (BEVY: POP CONTAINER COMPONENT)
    pub home: Option<Entity>,
}

impl Clone for Pop{
    fn clone(&self) -> Self {
        Pop {
            size: self.size,
            home: self.home,
        }
    }
}

impl Pop{
    /// Determine if these two pops are "equivalent" and able to be merged.
    pub fn mergeable(&self, other: &Mut<Pop>) -> bool {
        return true;
    }
}