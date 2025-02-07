use std::ops::Range;
use bevy::prelude::Vec2;
use poisson_diskus::bridson;
use spade::*;
use spade::handles::{DirectedVoronoiEdge, VoronoiVertex};
use crate::point::Point;

/// Generate some city center locations within a given area.
/// The city centers are at least radius apart.
pub fn generate_city_centers(area: [Range<f64>; 2], radius: f64) -> Vec<Vec2> {
    /* Use poisson disc sampling to generate a random assortment of points */
    let box_size = [area[0].end - area[0].start, area[1].end - area[1].start];
    let rmin = radius;
    let num_attempts = 30;
    let use_pbc = true;
    let coords = bridson(&box_size, rmin, num_attempts, use_pbc).unwrap();

    /* Convert coords into Vec2's */
    let mut out: Vec<Vec2>  = Vec::new();
    for coord in &coords{
        out.push(Vec2::new((coord[0] + area[0].start) as f32, (coord[1] + area[1].start) as f32));
    }

    /* Return the output */
    return out;
}

/// A container for generated map data.
pub struct ProvinceData{
    pub province_connections: Vec<(Vec2, Vec2)>,
    pub province_borders: Vec<(Vec2, Vec2)>,
}

/// From a set of cities, generate the bounds of their provinces with
/// respect to the map.
pub fn generate_provinces(cities: &Vec<Vec2>) -> ProvinceData {
    /* Begin by converting all cities to points */
    let mut points: Vec<Point> = Vec::new();
    for city in cities{
        points.push(Point{pos: *city });
    }

    /* Generate the triangulation generator*/
    let mut triangulation: DelaunayTriangulation<Point> = DelaunayTriangulation::new();

    /* Add points */
    for point in points{
        triangulation.insert(point).expect("PANIC");
    }

    /* First, grab all information related to pathfinding */
    let mut data = ProvinceData{
        province_borders: Vec::new(),
        province_connections: Vec::new(),
    };

    /* Return connections between provinces */
    for edge in triangulation.undirected_edges(){
        let a = edge.vertices()[0].data().pos;
        let b = edge.vertices()[1].data().pos;
        data.province_connections.push((a, b));
    }

    /* Return all edges */
    for cell in triangulation.voronoi_faces(){
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
                data.province_borders.push((a, b));
            }
        }
    }

    return data;
}