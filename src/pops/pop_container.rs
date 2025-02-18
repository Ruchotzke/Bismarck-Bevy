use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use crate::pops::pop::Pop;

/// A container with a city to hold the pops for that city.
#[derive(Component)]
pub struct PopContainer{
    /// The pops contained in this container
    pub pops:Vec<Entity>,
}

impl PopContainer{
    /// Construct a new, empty pop container.
    pub fn new() -> PopContainer{
        PopContainer {
            pops:Vec::new()
        }
    }

    /// Attempt to insert a new pop into this container.
    /// Returns true if a merge happened, otherwise false
    pub fn add_pop_and_merge(&mut self, mut pop: Pop, mut commands: Commands, mut query: Query<&mut Pop>, ent: Entity) {
        /* iterate through all pops; if we find a merge-able choice, merge. otherwise insert. */
        for existing_pop in self.pops.iter_mut() {
            let mut existing_pop = match query.get_mut(*existing_pop) {
                Ok(v) => {v}
                Err(_) => {
                    //TODO: THIS WILL FAIL IF ACCESSING AN ENTITY SPAWNED ON THE SAME TIMESTEP PRE-FLUSH (I THINK)
                    error!("WHOA! SOMEONE TRIED TO ACCESS THIS CONTAINER TWICE! (check todo in container)");
                    return;
                }
            };
            if pop.mergeable(&existing_pop){
                existing_pop.size += pop.size;
                return;
            }
        }

        /* Insert a new entity */
        pop.home = Some(ent);
        let pop_entity = commands.spawn(pop);
        self.pops.push(pop_entity.id());
    }
}