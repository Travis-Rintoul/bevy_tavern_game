use bevy::prelude::*;

use crate::core::{
    ALL_RECIPES, RecipeListOption, RecipeListOptionSelectedEvent, RecipeListWindow,
    RequestRecipeListUIPopulationEvent, RequestRecipeUIPopulationEvent,
};

pub fn option_selected_observer(trigger: Trigger<RecipeListOptionSelectedEvent>) {
    let _ = trigger;
}

pub fn on_populate_recipe_list_request(
    trigger: Trigger<RequestRecipeListUIPopulationEvent>,
    mut commands: Commands,
    children_query: Query<&Children>,
) {
    let target_entity = trigger.target();

    if let Ok(children) = children_query.get(target_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    let relavent_recipies = &ALL_RECIPES;

    commands.entity(target_entity).with_children(|parent| {
        relavent_recipies.iter().for_each(|recipe| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                Text::new(&recipe.name),
                TextColor(Color::srgb_u8(255, 255, 255)),
                RecipeListOption(recipe.id),
                Button,
            ));
        });
    });
}
