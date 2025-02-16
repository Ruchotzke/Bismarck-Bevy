use bevy::prelude::*;
use crate::rendering::map_views::map_view_state::MapModeState;

pub struct InputManager;

impl Plugin for InputManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_map_mode);
    }
}


fn handle_map_mode(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<MapModeState>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        /* Default mode */
        next_state.set(MapModeState::Flat);
    }
    else if keys.just_pressed(KeyCode::Digit2) {
        /* Pop density mode */
        next_state.set(MapModeState::PopulationDensity);
    }
}