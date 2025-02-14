mod world;
mod rendering;

use bevy::color::palettes::basic::{WHITE};
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;
use bevy_2d_line::{Line, LineRenderingPlugin};
use rand::Rng;
use crate::world::generate_world::generate_provinces;
use crate::world::province::Province;
use crate::world::worldgen::WorldGen;
use crate::world::city::City;
use crate::world::edge_marker::{mark_edge_provs, EdgeMarker};
use crate::world::neighbors::ProvinceNeighbors;

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
        .add_systems(Startup, (mark_edge_provs, remove_ugly_edges, render_world).chain().after(setup))

        .run();
}

fn setup(mut commands: Commands) {
    /* Set up a camera */
    commands.spawn((
        Camera2d::default(),
        Transform::from_scale(Vec3::new(1.1, 1.1, 1.1)), // Zooms in (values < 1.0)
    ));
}

fn render_world(
    mut commands: Commands,
    prov_query: Query<(&Province, &City, &ProvinceNeighbors)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (prov, city, neighbors) in prov_query.iter() {
        /* Render the city point */
        let city_color = Color::srgb(0.0, 0.0, 0.0);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(city_color)),
            Transform::from_xyz(city.pos.x, city.pos.y, 5.0)
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
        for entity in neighbors.prov_neighbors.iter() {
            /* Grab the city from this entity */
            let (_, dst, _) = prov_query.get(*entity).unwrap();

            /* Render the line */
            let points = vec![city.pos, dst.pos];
            let colors = vec![WHITE.into(), WHITE.into()];
            commands.spawn((Line {
                points,
                colors,
                thickness: 1.0,
            }, Transform::from_xyz(0.0, 0.0, 1.0))
            );
        }
    }
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

}