use bevy::prelude::*;
use crate::scheduling::startup_schedule::StartupSchedule;
use crate::world::city::City;
use rand::seq::IteratorRandom;
use crate::pops::add_pop::AddPopEvent;
use crate::pops::pop::Pop;
use crate::pops::pop_container::PopContainer;
use crate::world::neighbors::ProvinceNeighbors;

/// The plugin for managing populations.
pub struct PopManagement;

impl Plugin for PopManagement {
    fn build(&self, app: &mut App) {
        /* Add events */
        app.add_event::<AddPopEvent>();

        /* Add systems */
        app.add_systems(Startup,
                        (
                            initialize_pop_containers,
                            initialize_pops,
                        )
                            .chain()
                            .in_set(StartupSchedule::PopulationInitialization)
        );
        app.add_systems(Update,
                (
                            // update_pops,
                            handle_pop_adds,
                        )
        );
    }
}

pub fn initialize_pop_containers(mut commands: Commands, query: Query<Entity, With<City>>) {
    /* Add a pop container to all cities */
    for entity in query.iter(){
        commands.entity(entity).insert((
            PopContainer::new()
        ));
    }
}

pub fn initialize_pops(query: Query<Entity, With<PopContainer>>, mut ev_addpop: EventWriter<AddPopEvent>){
    /* Add a population to the some random provinces. */
    let entity = query.iter().choose(&mut rand::rng()).unwrap();
    let pop = Pop{size: 1500, home: None};
    ev_addpop.send(AddPopEvent{
        pop: pop.clone(),
        to_container: entity,
    });

    let entity = query.iter().choose(&mut rand::rng()).unwrap();
    let pop = Pop{size: 800, home: None};
    ev_addpop.send(AddPopEvent{
        pop: pop.clone(),
        to_container: entity,
    });

    let entity = query.iter().choose(&mut rand::rng()).unwrap();
    let pop = Pop{size: 200, home: None};
    ev_addpop.send(AddPopEvent{
        pop: pop.clone(),
        to_container: entity,
    });

}

pub fn update_pops(mut pops: Query<(Entity, &mut Pop)>,
                   mut cities: Query<(Entity, &mut PopContainer, &ProvinceNeighbors)>,
                   time: Res<Time>,
                   mut ev_addpop: EventWriter<AddPopEvent>
) {
    for(entity, mut pop) in pops.iter_mut() {
        /* Get this pop's neighbor information */
        let (_, container, neighbors) = cities.get_mut(pop.home.unwrap()).unwrap();

        /* Compute the amount of pop to reduce here */
        let reduction: u32 = (pop.size as f32 * 0.4 * time.delta_secs().ceil()) as u32;

        /* Distribute reduction between all neighbors */
        let delta: u32 = (reduction / neighbors.prov_neighbors.len() as u32);

        /* Reduce population */
        pop.size -= reduction;

        /* If pop is zero, delete it and remove the reference from the list */
        // if pop.size == 0 {
        //     commands.entity(entity).despawn();
        //     let index = container.pops.iter().position(|x| *x == pop.home.unwrap()).unwrap();
        //     container.pops.remove(index);
        // }

        /* Create a pop at all neighbors */
        for neighbor in neighbors.prov_neighbors.iter() {
            let pop = Pop{size: delta, home: None};

            // let (e, mut nc, _) = cities.get_mut(*neighbor).unwrap();
            // ev_addpop.send(AddPopEvent{
            //     pop,
            //     to_container: e,
            // });
        }
    }
}

fn handle_pop_adds(
    mut commands: Commands,
    mut ev_addpop: EventReader<AddPopEvent>,
    mut query: Query<(&mut PopContainer)>,
    mut pops: Query<&mut Pop>
) {
    for mut ev in ev_addpop.read() {
        /* Get the corresponding pop container for this new pop */
        let mut container = query.get_mut(ev.to_container).unwrap();

        /* Add the pop to the container */
        container.add_pop_and_merge(ev.pop.clone(), commands.reborrow(), pops.reborrow(), ev.to_container);
    }
}