mod world;
mod rendering;
mod pops;
mod scheduling;

use bevy::prelude::*;
use bevy_2d_line::{LineRenderingPlugin};
use crate::world::worldgen::{WorldGen};
use bevy::app::Startup;
use crate::pops::pop_manager::PopManagement;
use crate::rendering::render_plugin::RenderPlugin;
use crate::scheduling::startup_schedule::StartupSchedule;

fn main() {
    /* Generate the app */
    let mut app = App::new();

    /* Configure subschedule sets */
    app.configure_sets(Startup,
                       (
                           StartupSchedule::WorldGeneration,
                           StartupSchedule::PopulationInitialization,
                           StartupSchedule::RenderInitialization
                       ).chain());

    /* Third party plugins */
    app.add_plugins(DefaultPlugins)
    .add_plugins(LineRenderingPlugin);

    /* Custom plugins */
    app.add_plugins(WorldGen)
    .add_plugins(PopManagement)
    .add_plugins(RenderPlugin);

    /* Resources */
    app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)));

    /* Run the app */
    app.run();
}