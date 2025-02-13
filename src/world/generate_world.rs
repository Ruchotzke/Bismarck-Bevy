use std::ops::Range;
use bevy::prelude::*;
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
        triangulation,
    });
}

pub fn connect_provinces(mut query: Query<(Entity, &mut Province)>, graph: Res<MapTriangulation>) {
    /* Save all of the provinces and vertices */
    let provs: Vec<(Entity, Mut<Province>)> = query.iter_mut().collect();

    /* Collect all cities, maintaining order */
    let mut cities: Vec<Vec2> = Vec::new();
    for (_, p) in &provs {
        cities.push(p.city.clone());
    }

    /* Get the mapping from city to a list of vec2 neighbors */
    let tmp_store = get_prov_neighbors(&cities, graph);

    /* Convert Vec2 into entity */
    let mut tmp_entities: Vec<(Vec2, Vec<Entity>)> = Vec::new();
    for (city, vecs) in tmp_store {
        let mut neighbors = Vec::new();
        for v in vecs.iter() {
            let (e, _) = provs.iter().find(|&(_, p)| p.city == *v).unwrap();
            neighbors.push(*e);
        }
        info!("City {} has {} neighbors.", city, neighbors.len());
        tmp_entities.push((city, neighbors));
    }

    /* Now update each province with its corresponding neighboring entities */
    for (_, mut p) in provs {
        /* Find the correct province mapping */
        let (_, ents) = tmp_entities.iter().find(|(city, _)| *city == p.city).unwrap();

        /* Update the province */
        for e in ents {
            p.neighboring_provinces.push(*e);
        }
    }
}


/// Map a province to its neighboring cities (helper)
fn get_prov_neighbors(cities: &Vec<Vec2>, graph: Res<MapTriangulation>) -> Vec<(Vec2, Vec<Vec2>)> {
    /* Create a read-only mapping to store neighbors temporarily */
    let mut tmp_store: Vec<(Vec2, Vec<Vec2>)> = Vec::new();
    let mut vertices = graph.triangulation.vertices().collect::<Vec<_>>();

    /* Compute neighbors */
    for city in cities.iter() {
        /* Find the corresponding vertex in the triangulation */
        let vertex = vertices.iter_mut().find(|v| v.data().pos == *city).unwrap();

        /* Traverse vertex neighbors */
        let mut neighbors = Vec::new();
        for edge in vertex.out_edges() {
            let dst = edge.to().data().pos;
            neighbors.push(dst);
        }

        /* Store neighbors */
        tmp_store.push((city.clone(), neighbors));
    }
    return tmp_store;
}