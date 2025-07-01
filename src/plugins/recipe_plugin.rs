use bevy::prelude::*;

use crate::core::{
    InterfaceFlowSet, Owner, Player, PlayerOpenedDeviceInterfaceEvent, RecipeListWindow,
    RecipeWindowPopulationRequestEvent,
};

pub struct RecipePlugin;

impl Plugin for RecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            emit_recipe_window_population_request.in_set(InterfaceFlowSet::AfterChange),
        );
    }
}

fn emit_recipe_window_population_request(
    player_query: Query<Entity, With<Player>>,
    mut events: EventReader<PlayerOpenedDeviceInterfaceEvent>,
    recipe_window_query: Query<(Entity, Option<&Owner>), With<RecipeListWindow>>,
    mut population_request_event: EventWriter<RecipeWindowPopulationRequestEvent>,
) {
    for event in events.read() {
        let Ok(player_entity) = player_query.single() else {
            // TODO: make message a constant
            panic!("Unable to find player");
        };

        for (window_entity, owner) in &recipe_window_query {
            if let Some(Owner::Device(device_entity)) = owner {
                if *device_entity == event.device {
                    // Tell the UI to display the ui items
                    population_request_event.write(RecipeWindowPopulationRequestEvent {
                        window_entity: window_entity,
                        inventory_entity: player_entity,
                        device_type: event.device_type.clone(),
                    });
                }
            }
        }
    }
}
