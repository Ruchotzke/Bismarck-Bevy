use std::ops::Range;
use bevy::prelude::{Commands, Entity, Res, Vec2};
use poisson_diskus::bridson;
use spade::*;
use crate::world::province::Province;
use crate::world::triangulation::{MapTriangulation, Point};
use crate::world::world_config::WorldConfig;

/// Generate some city center locations within a given area.
/// The city centers are at least radius apart.
fn generate_city_centers(area: &[Range<f64>; 2], radius: f64) -> Vec<Vec2> {
    /* Use poisson disc sampling to generate a random assortment of points */
    let box_size = [area[0].end - area[0].start, area[1].end - area[1].start];
    let rmin = radius;
    let num_attempts = 30;
    let use_pbc = true;
    let coords = bridson(&box_size, rmin, num_attempts, use_pbc).unwrap();

    /* Convert coords into Vec2's and provinces */
    let mut out: Vec<Vec2>  = Vec::new();
    for coord in &coords{
        out.push(Vec2::new((coord[0] + area[0].start) as f32, (coord[1] + area[1].start) as f32));
    }

    /* Return the output */
    return out;
}

/// From a set of cities, generate the bounds of their provinces with
/// respect to the map.
pub fn generate_provinces(mut commands: Commands, config: Res<WorldConfig>) {
    /* Generate city centers */
    let pos = generate_city_centers(&config.world_area, config.city_pos_radius);

    /* Handle a triangulation */
    let mut triangulation: DelaunayTriangulation<Point> = DelaunayTriangulation::new();
    for point in pos {
        triangulation.insert(Point{pos: point}).expect("Failed to insert point");
    }

    /* Maintain a set of entities */
    let mut provs: Vec<Entity> = Vec::new();

    /* Generate a province for each voronoi face */
    for cell in triangulation.voronoi_faces(){
        let mut edges = Vec::new();
        for edge in cell.adjacent_edges() {
            /* Unwrap if possible (may be an outer vertex */
            let mut bad = false;
            let a: Vec2 = match edge.from().position() {
                None => {bad = true; Vec2::ZERO},
                Some(v) => {Vec2::new(v.x, v.y)}
            };
            let b: Vec2 = match edge.to().position() {
                None => {bad = true; Vec2::ZERO},
                Some(v) => {Vec2::new(v.x, v.y)}
            };

            if !bad {
                edges.push((a,b));
            }
        }

        /* If there are any edges, make a province */
        if edges.len() > 0 {
            /* Spawn the province */
            let prov = Province{
                city: Vec2::new(cell.as_delaunay_vertex().position().x, cell.as_delaunay_vertex().position().y),
                borders: edges,
                neighboring_provinces: vec![],
            };
            let entity = commands.spawn(
                prov
            );
            provs.push(entity.id());
        }
    }

    /* For future use, save the triangulation as a resource */
    commands.insert_resource(MapTriangulation{
        triangulation: triangulation,
    });

    // /* Begin by converting all cities to points */
    // let mut points: Vec<Point> = Vec::new();
    // for prov in provs{
    //     points.push(Point{pos: *prov.city.center });
    // }
    //
    // /* Generate the triangulation generator*/
    // let mut triangulation: DelaunayTriangulation<Point> = DelaunayTriangulation::new();
    //
    // /* Add points */
    // for point in points{
    //     triangulation.insert(point).expect("PANIC");
    // }
    //
    // /* First, grab all information related to pathfinding */
    // let mut data = ProvinceData{
    //     province_borders: Vec::new(),
    //     province_connections: Vec::new(),
    // };
    //
    // /* Return connections between provinces */
    // for edge in triangulation.undirected_edges(){
    //     let a = edge.vertices()[0].data().pos;
    //     let b = edge.vertices()[1].data().pos;
    //     data.province_connections.push((a, b));
    // }
    //
    // /* Return all edges */
    // for cell in triangulation.voronoi_faces(){
    //     for edge in cell.adjacent_edges() {
    //         /* Unwrap if possible (may be an outer vertex */
    //         let mut bad = false;
    //         let a: Vec2 = match edge.from().position() {
    //             None => {bad = true; Vec2::ZERO},
    //             Some(v) => {Vec2::new(v.x, v.y)}
    //         };
    //         let b: Vec2 = match edge.to().position() {
    //             None => {bad = true; Vec2::ZERO},
    //             Some(v) => {Vec2::new(v.x, v.y)}
    //         };
    //         if !bad {
    //             data.province_borders.push((a, b));
    //         }
    //     }
    // }
    //
    // return data;
}