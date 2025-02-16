use bevy::prelude::*;
use crate::pops::pop::Pop;

#[derive(Event)]
pub struct AddPopEvent {
    pub pop: Pop,
    pub to_container: Entity,
}