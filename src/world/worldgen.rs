use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use crate::scheduling::startup_schedule::StartupSchedule;
use crate::world::edge_marker::{mark_edge_provs, EdgeMarker};
use crate::world::generate_world::{generate_provinces, connect_provinces};
use crate::world::neighbors::ProvinceNeighbors;
use crate::world::world_config::WorldConfig;

pub struct WorldGen;

/// The world generation schedule label (to ensure it runs early)
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitWorldSched;

impl Plugin for WorldGen {
    fn build(&self, app: &mut App) {

        /* Add systems */
        app.add_systems(Startup,
                        (init_worldgen,
                         generate_provinces,
                         connect_provinces,
                         mark_edge_provs,
                         remove_ugly_edges,
                         conclude_worldgen
                        )
                            .chain()
                            .in_set(StartupSchedule::WorldGeneration));
    }
}

fn init_worldgen(mut commands: Commands) {
    /* Generate the config */
    commands.insert_resource(WorldConfig::new());
}

fn remove_ugly_edges(
    mut commands: Commands,
    mut params: ParamSet<(
        Query<(Entity, &ProvinceNeighbors, &EdgeMarker)>,
        Query<&mut ProvinceNeighbors>
    )>
){
    /* First find all entities we want to remove */
    let mut to_remove = Vec::new();
    let binding = params.p0();
    let all = binding.iter().collect::<Vec<_>>();
    for (e, _, _) in all {
        to_remove.push(e);
    }

    /* Now iterate through all neighbor structs and remove that entity */
    for check in &to_remove {
        for mut province in params.p1().iter_mut() {
            for i in 0..province.prov_neighbors.len() {
                if province.prov_neighbors[i] == *check {
                    province.prov_neighbors.remove(i);
                    break;
                }
            }
        }
    }

    /* Fully delete the entity */
    for r in to_remove {
        commands.entity(r).despawn();
    }

    info!("Completed bad edge removal.");

}

pub fn conclude_worldgen() {
    /* Any cleanup needed after worldgen is done. */
}