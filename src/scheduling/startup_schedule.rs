use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSchedule {
    WorldGeneration,            // All world-gen items
    PopulationInitialization    // Initialization of world populations
}