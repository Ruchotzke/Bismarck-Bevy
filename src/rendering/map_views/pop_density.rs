use std::collections::HashMap;
use bevy::color::palettes::basic::{GRAY, GREEN};
use bevy::prelude::*;
use crate::pops::pop::Pop;
use crate::world::city::City;
use crate::world::province::Province;

/// Render the map colors as population density per province.
pub fn render_pop_density(city_query: Query<(&MeshMaterial2d<ColorMaterial>, Entity), With<City>>,
                          mut materials: ResMut<Assets<ColorMaterial>>,
                          pop_query: Query<&Pop>,
){
    /* First, compute the minimum and maximum province populations to adjust our range */
    let mut map: HashMap<Entity, u64> = HashMap::new();
    for pop in pop_query.iter(){
        match map.get_mut(&pop.home.unwrap()){
            None => {
                map.insert(pop.home.unwrap(), pop.size as u64);
            }
            Some(v) => {
                *v += pop.size as u64;
            }
        }
    }
    let mut min: u64 = u64::MAX;
    let mut max: u64 = u64::MIN;
    for (_, value) in &map {
        if value < &min {
            min = *value;
        }
        if value > &max {
            max = *value;
        }
    }

    for (handle, entity) in city_query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            /* Modulate the material color between gray and green depending on pop */
            /* Collect population */
            let pop = match map.get(&entity) {
                None => {0}
                Some(v) => {*v}
            };
            let norm:f32 = pop as f32 / max as f32;

            /* Interpolate color */
            let color = color_lerp(GRAY, Srgba::rgb(0.0, 1.0, 0.0), norm);

            /* Color province */
            material.color = color;
        }
    }
}

fn color_lerp(a: Srgba, b: Srgba, t: f32) -> Color {
    let aa: Vec4 = Vec4::new(a.red, a.green, a.blue, a.alpha);
    let bb: Vec4 = Vec4::new(b.red, b.green, b.blue, b.alpha);

    let cc = aa.lerp(bb, t);
    return Color::srgba(cc.x, cc.y, cc.z, cc.w);
}