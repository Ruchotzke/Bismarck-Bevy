use bevy::prelude::*;
use crate::scheduling::startup_schedule::StartupSchedule;
use crate::world::city::City;

/// The plugin for managing populations.
pub struct PopManagement;

impl Plugin for PopManagement {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
                        (
                            initialize_pops
                        )
                            .chain()
                            .in_set(StartupSchedule::PopulationInitialization));
    }
}

pub fn initialize_pops(mut commands: Commands, query: Query<(Entity, &City)>){
    /* Add a population to the zero'th province. */
    info!("Initializing population!");
}

