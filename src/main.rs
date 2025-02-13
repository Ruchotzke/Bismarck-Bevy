mod world;
mod rendering;

use bevy::color::palettes::basic::{RED, WHITE};
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;
use bevy_2d_line::{Line, LineRenderingPlugin};
use rand::Rng;
use crate::world::generate_world::generate_provinces;
use crate::world::province::Province;
use crate::world::worldgen::WorldGen;
use crate::rendering::{convex_polygon};
use crate::rendering::convex_polygon::generate_convex_mesh;
use crate::rendering::ordering::{order_verts, remove_duplicate_verts};

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
        // .add_systems(Startup, render_test)

        .run();
}

fn setup(mut commands: Commands) {
    /* Set up a camera */
    commands.spawn(Camera2d);
}

fn render_world(
    mut commands: Commands,
    prov_query: Query<&Province>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for prov in prov_query.iter() {
        /* Render the city point */
        let city_color = Color::srgb(0.0, 0.0, 0.0);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(city_color)),
            Transform::from_xyz(prov.city.x, prov.city.y, 1.0)
        ));

        /* Render the province area */
        let rand_color = Color::hsv(rand::rng().random_range(0.0..360.0), 1.0, 1.0);
        let (worked, mesh) = match prov.generate_flat_mesh() {
            Ok(mesh) => (true, Some(mesh)),
            Err(_e) => (false, None)
        };
        if worked {
            commands.spawn((
                Mesh2d(meshes.add(mesh.unwrap())),
                MeshMaterial2d(materials.add(rand_color)),
                Transform::from_xyz(0.0, 0.0, -1.0)
            ));
        }

        for edge in &prov.borders {
            let (a, b) = *edge;
            let points = vec![a, b];
            let colors = vec![BLACK.into(), BLACK.into()];
            commands.spawn(Line {
                points,
                colors,
                thickness: 3.0,
            });
        }

        /* Render the province neighbors */
        for entity in prov.neighboring_provinces.iter() {
            /* Grab the city from this entity */
            let dst = prov_query.get(*entity).unwrap().city;

            /* Render the line */
            let points = vec![prov.city, dst];
            let colors = vec![WHITE.into(), WHITE.into()];
            commands.spawn(Line {
                points,
                colors,
                thickness: 1.0,
            });
        }
    }
}

// fn render_test(mut commands: Commands,
//                mut meshes: ResMut<Assets<Mesh>>,
//                mut materials: ResMut<Assets<ColorMaterial>>) {
//
//     /* Spawn a new mesh */
//     let color = Color::srgb(1.0, 1.0, 0.0);
//     let verts = vec![
//         Vec2::new(0.0, 0.0),
//         Vec2::new(100.0, 0.0),
//         Vec2::new(45.9, 45.9),
//         Vec2::new(0.0, 100.0),
//         Vec2::new(-70.0, 80.0),
//         Vec2::new(-90.0, 20.0),
//     ];
//     let verts = remove_duplicate_verts(verts);
//     commands.spawn((
//         Mesh2d(meshes.add(generate_convex_mesh(
//             verts,
//         ).unwrap())
//         ),
//         MeshMaterial2d(materials.add(color)),
//         Transform::from_xyz(0.0, 0.0, 0.0)
//     ));
// }