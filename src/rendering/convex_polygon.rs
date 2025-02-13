use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::rendering::ordering::order_verts;

/// Generate a 2D mesh from a series of convex 2D points
pub fn generate_convex_mesh(verts: Vec<Vec2>) -> Result<Mesh, String> {
    if verts.len() < 3 {
        return Err(String::from("Not enough verts"));
    }

    /* Ensure the verts are in CCW winding order */
    let verts = order_verts(verts);

    /* Starting from a vertex, create a triangle fan */
    let mut positions = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals = Vec::new();
    let mut uv = Vec::new();

    /* Insert vertices and normals*/
    for v in &verts {
        positions.push([v.x, v.y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uv.push([0.0, 0.0]);
    }

    /* Update indices */
    for i in 1..(&verts.len()-1) {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i+1) as u32);
    }

    /* Generate the mesh */
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.insert_indices(Indices::U32(indices));

    return Ok(mesh);
}