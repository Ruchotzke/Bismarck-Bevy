use bevy::prelude::*;
use crate::world::generate_world::{generate_provinces, connect_provinces};
use crate::world::world_config::WorldConfig;

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_worldgen, generate_provinces, connect_provinces).chain());
    }
}

fn init_worldgen(mut commands: Commands) {
    /* Generate the config */
    commands.insert_resource(WorldConfig::new());
}