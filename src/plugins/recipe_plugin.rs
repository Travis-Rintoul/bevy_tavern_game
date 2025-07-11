use bevy::{ecs::system::command, prelude::*};

use crate::core::{
    InterfaceFlowSet, Owner, Player, PlayerOpenedDeviceUIEvent, RecipeID, RecipeListOption,
    RecipeListOptionSelectedEvent, RecipeListWindow, RequestRecipeListUIPopulationEvent,
    RequestRecipeUIPopulationEvent,
};

pub struct RecipePlugin;

impl Plugin for RecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_recipe_selected);
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
