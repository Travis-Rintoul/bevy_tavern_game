use bevy::{ecs::system::command, prelude::*};

use crate::core::{
    InterfaceFlowSet, Owner, Player, PlayerOpenedDeviceUIEvent, RecipeID, RecipeListOption,
    RecipeListOptionSelectedEvent, RecipeListWindow, RequestRecipeListUIPopulationEvent,
    RequestRecipeUIPopulationEvent,
};

pub struct RecipePlugin;

impl Plugin for RecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_recipe_selected,
                emit_recipe_window_population_request.in_set(InterfaceFlowSet::AfterHook),
            ),
        );
    }
}

fn emit_recipe_window_population_request(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut events: EventReader<PlayerOpenedDeviceUIEvent>,
    recipe_window_query: Query<(Entity, Option<&Owner>), With<RecipeListWindow>>,
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
                    commands
                        .entity(window_entity)
                        .trigger(RequestRecipeListUIPopulationEvent {
                            inventory_entity: player_entity,
                            device_type: event.device_type.clone(),
                        });
                }
            }
        }
    }
}

fn handle_recipe_selected(
    mut query: Query<(&Interaction, &RecipeListOption, &ChildOf), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, button_data, parent) in &mut query {
        if *interaction == Interaction::Pressed {
            commands
                .entity(parent.0)
                .trigger(RecipeListOptionSelectedEvent {
                    recipe_id: button_data.0,
                });
        }
    }
}

fn handle_recipe_craft_request() {}
