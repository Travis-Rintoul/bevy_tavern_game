use bevy::prelude::*;

use crate::{
    core::{
        ActiveDeviceResource, Crafting, CraftingStation, Owner, RecipeListWindowOptionSelected,
        RecipeWindow, RecipeWindowPopulationRequestEvent, UIState,
    },
    ui::{RecipeListWindowBundle, RecipeWindowBundle},
};

pub fn setup(mut commands: Commands, active_device: Res<ActiveDeviceResource>) {
    let Some(entity) = active_device.0 else {
        return;
    };

    commands
        .spawn((
            Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            StateScoped(UIState::DeviceStove),
            BackgroundColor(Color::srgb_u8(255, 0, 0)),
            CraftingStation::default(),
            Crafting::default(),
        ))
        .with_children(|parent| {
            // Side Bar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    RecipeListWindowBundle::default(),
                    Owner::Device(entity),
                ))
                .observe(show_selected_recipe);

            // Main bar
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                RecipeWindowBundle::default(), // Crafting button
                Owner::Device(entity),
            ));
        });
}

fn show_selected_recipe(
    trigger: Trigger<RecipeListWindowOptionSelected>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    sibling_query: Query<&Children>,
    recipe_window_query: Query<&RecipeWindow>,
) {
    let target_entity = trigger.target();

    // TODO: error handle this properly
    let Some(parent) = parent_query.get(target_entity).ok() else {
        return;
    };

    // TODO: error handle this properly
    let Some(children) = sibling_query.get(parent.0).ok() else {
        return;
    };

    for child_entity in children.iter() {
        if recipe_window_query.get(child_entity).is_ok() {
            commands
                .entity(child_entity)
                .trigger(RecipeWindowPopulationRequestEvent {
                    recipe_id: trigger.event().recipe_id,
                });
        }
    }
}
