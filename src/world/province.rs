use bevy::prelude::*;
use crate::rendering::convex_polygon::generate_convex_mesh;
use crate::rendering::ordering::remove_duplicate_verts;
use crate::world::city::City;
use crate::world::neighbors::ProvinceNeighbors;

#[derive(Bundle)]
pub struct ProvinceBundle{
    pub city: City,
    pub geography: Province,
    pub neighbors: ProvinceNeighbors
}

#[derive(Component, Debug)]
pub struct Province {
    /// The borders in 2D space for this province
    pub borders: Vec<(Vec2, Vec2)>,
}

impl Province {
    pub fn new(edges: Vec<(Vec2, Vec2)>) -> Self{
        Province { borders: edges }
    }

    pub fn generate_flat_mesh(&self) -> Result<Mesh, String> {
        /* Collect all vertices of this border */
        let mut verts = Vec::new();
        for (a, b) in self.borders.iter() {
            verts.push(*a);
            verts.push(*b);
        }

        /* Remove duplicates */
        let verts = remove_duplicate_verts(verts);

        /* Generate the mesh */
        return generate_convex_mesh(verts);
    }
}