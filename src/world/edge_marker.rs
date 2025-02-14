use bevy::prelude::*;
use crate::world::city::City;
use crate::world::province::Province;
use crate::world::world_config::WorldConfig;


#[derive(Component, Debug)]
pub struct EdgeMarker;

pub fn mark_edge_provs(mut commands: Commands, query: Query<(Entity, &Province, &City)>, config: Res<WorldConfig>) {
    for (entity, province, city) in query.iter() {
        for edge in &province.borders {
            if edge.0.distance(city.pos) > 1.2 * config.city_pos_radius as f32 {
                commands.entity(entity).insert(EdgeMarker);
                break;
            }
        }
    }
}