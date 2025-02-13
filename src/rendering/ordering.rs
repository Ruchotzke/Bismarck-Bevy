use std::ptr::null;
use bevy::prelude::*;
use bevy::prelude::ops::atan2;

/// Order these verts in CCW order.
pub fn order_verts(verts: Vec<Vec2>) -> Vec<Vec2> {
    /* First compute the centroid */
    let mut centroid = Vec2::ZERO;
    for vert in &verts {
        centroid += vert;
    }
    centroid /= verts.len() as f32;

    /* Compute the relative angle of all verts to the centroid */
    let mut angles: Vec<(Vec2, f32)> = Vec::new();
    for vert in &verts {
        let angle = atan2(vert.y - centroid.y, vert.x - centroid.x);
        angles.push((vert.clone(), angle));
    }

    /* Sort based on angle */
    angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    /* Generate an array to return */
    let mut ret:Vec<Vec2> = Vec::new();
    for set in angles {
        ret.push(set.0);
    }
    return ret;
}

/// Remove duplicate vertices from the provided array
pub fn remove_duplicate_verts(verts: Vec<Vec2>) -> Vec<Vec2>{
    let mut ret:Vec<Vec2> = Vec::new();

    for vert in verts {
        if !ret.contains(&vert) {
            ret.push(vert);
        }
    }

    return ret;
}