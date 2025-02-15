use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapModeState{
    #[default]
    Flat                    = 0,
    PopulationDensity       = 1,
}