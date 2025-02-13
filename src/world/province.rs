use bevy::prelude::*;
use crate::rendering::convex_polygon::generate_convex_mesh;
use crate::rendering::ordering::remove_duplicate_verts;

#[derive(Component, Debug)]
pub struct Province {
    /// The central city for this province
    pub city: Vec2,

    /// The borders in 2D space for this province
    pub borders: Vec<(Vec2, Vec2)>,

    /// The set of neighboring provinces to this one
    pub neighboring_provinces: Vec<Entity>,
}

impl Province {
    pub fn new() -> Self {
        Province {
            city: Vec2::ZERO,
            borders: Vec::new(),
            neighboring_provinces: Vec::new(),
        }
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