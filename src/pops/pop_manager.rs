use bevy::prelude::*;
use crate::scheduling::startup_schedule::StartupSchedule;
use crate::world::city::City;
use rand::seq::IteratorRandom;
use crate::pops::pop::Pop;

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
    let (entity, _) = query.iter().choose(&mut rand::rng()).unwrap();

    commands.spawn(
        Pop{
            size: 500,
            home: entity,
        },
    );
}

