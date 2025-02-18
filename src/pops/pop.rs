use bevy::prelude::*;

#[derive(Component, Copy)]
pub struct Pop{
    /// The number of people in this population.
    pub size: u32,

    /// The fractional component of the pop size (used to ensure consistency over time
    /// when moving pops).
    pub frac_size: f32,

    /// The home city of this population
    /// (BEVY: POP CONTAINER COMPONENT)
    pub home: Option<Entity>,
}

impl Clone for Pop{
    fn clone(&self) -> Self {
        Pop {
            size: self.size,
            frac_size: self.frac_size,
            home: self.home,
        }
    }
}

impl Pop{
    /// Determine if these two pops are "equivalent" and able to be merged.
    pub fn mergeable(&self, other: &Mut<Pop>) -> bool {
        return true;
    }

    /// Helper to help handle fractional reduction of pop size.
    pub fn reduce_population(&mut self, amt: f32) {
        /* First remove the whole number */
        self.size -= amt.floor() as u32;

        /* Remove the fractional part and reduce size if needed */
        if amt.fract() > self.frac_size {
            self.size -= 1;
            self.frac_size = 1.0 - (self.frac_size - amt.fract());
        }
        else {
            self.frac_size -= amt.fract();
        }
    }
}