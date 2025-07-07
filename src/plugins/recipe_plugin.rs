use bevy::{ecs::system::command, prelude::*};

use crate::core::{
    InterfaceFlowSet, Owner, Player, PlayerOpenedDeviceInterfaceEvent, RecipeID, RecipeListOption,
    RecipeListWindow, RecipeListWindowOptionSelected, RecipeListWindowPopulationRequestEvent,
    RecipeWindowPopulationRequestEvent,
};

pub struct RecipePlugin;

impl Plugin for RecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_recipe_selected,
                emit_recipe_window_population_request.in_set(InterfaceFlowSet::AfterChange),
            ),
        );
    }
}

fn emit_recipe_window_population_request(
    mut commands: Commands,
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
                    commands.entity(window_entity).trigger(
                        RecipeListWindowPopulationRequestEvent {
                            inventory_entity: player_entity,
                            device_type: event.device_type.clone(),
                        },
                    );
                }
            }
        }
    }
}

fn handle_recipe_selected(
    mut command: Commands,
    mut query: Query<(&Interaction, &RecipeListOption, &ChildOf, Entity), Changed<Interaction>>,
    parent_query: Query<&Children>,
    mut commands: Commands,
) {
    for (interaction, button_data, parent, entity) in &mut query {
        if *interaction == Interaction::Pressed {
            commands
                .entity(parent.0)
                .trigger(RecipeListWindowOptionSelected {
                    recipe_id: button_data.0,
                });
        }
    }
}

fn handle_recipe_craft_request() {}
