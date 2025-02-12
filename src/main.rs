mod world;

use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_2d_line::{Line, LineRenderingPlugin};
use crate::world::generate_world::generate_provinces;
use crate::world::province::Province;
use crate::world::worldgen::WorldGen;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))

        /* Third party plugins */
        .add_plugins(DefaultPlugins)
        .add_plugins(LineRenderingPlugin)

        /* Custom plugins */
        .add_plugins(WorldGen)

        /* Systems */
        .add_systems(Startup, setup.after(generate_provinces))
        .add_systems(Startup, render_world.after(setup))

        .run();
}

fn setup(mut commands: Commands) {
    /* Set up a camera */
    commands.spawn(Camera2d);

    // for edge in &prov_data.province_connections {
    //     let (a, b) = *edge;
    //     let points = vec![a, b];
    //     let colors = vec![ORANGE.into(), ORANGE.into()];
    //     commands.spawn(Line {
    //         points,
    //         colors,
    //         thickness: 1.0,
    //     });
    // }
}

fn render_world(
    mut commands: Commands,
    prov_query: Query<&Province>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for prov in prov_query.iter() {
        /* Render the city point */
        let city_color = Color::srgb(1.0, 1.0, 0.0);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(city_color)),
            Transform::from_xyz(prov.city.x, prov.city.y, 0.0)
        ));

        /* Render the province area */
        for edge in &prov.borders {
            let (a, b) = *edge;
            let points = vec![a, b];
            let colors = vec![RED.into(), RED.into()];
            commands.spawn(Line {
                points,
                colors,
                thickness: 1.0,
            });
        }

        /* Render the province neighbors */
        // info!("Prov {} has {} neighbors.", prov.city, prov.neighboring_provinces.len());
        for entity in prov.neighboring_provinces.iter() {
            /* Grab the city from this entity */
            let dst = prov_query.get(*entity).unwrap().city;

            /* Render the line */
            let points = vec![prov.city, dst];
            let colors = vec![RED.into(), RED.into()];
            commands.spawn(Line {
                points,
                colors,
                thickness: 1.0,
            });
        }
    }
}