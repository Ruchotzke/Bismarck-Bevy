use bevy::color::palettes::basic::{BLACK, WHITE};
use bevy::prelude::*;
use bevy_2d_line::Line;
use rand::Rng;
use crate::rendering::map_views::pop_density::render_pop_density;
use crate::scheduling::startup_schedule::StartupSchedule;
use crate::world::city::City;
use crate::world::neighbors::ProvinceNeighbors;
use crate::world::province::Province;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
                        (
                            construct_render_assets
                        )
                            .chain()
                            .in_set(StartupSchedule::RenderInitialization));
        app.add_systems(Update, render_pop_density);
    }
}

fn construct_render_assets(
    mut commands: Commands,
    prov_query: Query<(&Province, &City, &ProvinceNeighbors, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    /* Set up a camera */
    commands.spawn((
        Camera2d::default(),
        Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)), // Zooms in (values < 1.0)
    ));

    /* Handle map */
    for (prov, city, neighbors, entity) in prov_query.iter() {
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
            commands.entity(entity).insert(
                (
                    Mesh2d(meshes.add(mesh.unwrap())),
                    MeshMaterial2d(materials.add(rand_color)),
                    Transform::from_xyz(0.0, 0.0, -1.0)
                )
            );
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
            let (_, dst, _, _) = prov_query.get(*entity).unwrap();

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

fn adjust_color(mut materials: ResMut<Assets<ColorMaterial>>,
                query: Query<&MeshMaterial2d<ColorMaterial>, With<City>>,
                time: Res<Time>
){
    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.color = material.color.rotate_hue(time.delta_secs() * 100.0);
        }
    }
}