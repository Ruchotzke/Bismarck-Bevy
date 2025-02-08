mod point;
mod map_gen;

use bevy::color::palettes::basic::RED;
use bevy::color::palettes::css::ORANGE;
use bevy::prelude::*;
use bevy::reflect::GetTupleField;
use bevy_2d_line::{Line, LineRenderingPlugin};
use spade::{DelaunayTriangulation, Point2, Triangulation};
use crate::map_gen::generate_world::{generate_city_centers, generate_provinces};
use crate::point::Point;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))

        /* Third party plugins */
        .add_plugins(DefaultPlugins)
        .add_plugins(LineRenderingPlugin)

        /* Custom plugins */

        /* Systems */
        .add_systems(Startup, setup)

        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    /* Set up a camera */
    commands.spawn(Camera2d);

    /* Generate a set of city points */
    let cities = generate_city_centers([-600.0..600.0, -350.0..350.0], 30.0);

    /* Render those city points */
    let city_color = Color::srgb(1.0, 1.0, 0.0);
    for city in &cities {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(3.0))),
            MeshMaterial2d(materials.add(city_color)),
            Transform::from_xyz(city.x, city.y, 0.0),
        ));
    }

    /* Render the triangulation */
    let prov_data = generate_provinces(&cities);
    for edge in &prov_data.province_borders {
        let (a, b) = *edge;
        let points = vec![a, b];
        let colors = vec![RED.into(), RED.into()];
        commands.spawn(Line {
            points,
            colors,
            thickness: 1.0,
        });
    }
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