use bevy::prelude::*;

use crate::core::{ALL_RECIPES, CraftButton, RequestRecipeUIPopulationEvent};

pub struct RecipeWindowUIElementPlugin;

impl Plugin for RecipeWindowUIElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(populate_recipe_window);
    }
}

fn populate_recipe_window(
    trigger: Trigger<RequestRecipeUIPopulationEvent>,
    mut commands: Commands,
    children_query: Query<&Children>,
) {
    let event = trigger.event();

    // TODO: add nice recipe lookup
    let Some(recipe) = ALL_RECIPES
        .iter()
        .find(|recipe| recipe.id == trigger.event().recipe_id)
    else {
        return;
    };

    let target_entity = trigger.target();

    if let Ok(children) = children_query.get(target_entity) {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }

    commands.entity(target_entity).with_children(|parent| {
        parent.spawn(Text::new(&recipe.name));
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                right: Val::Px(0.0),
                ..Default::default()
            },
            Button,
            Text::new("Cook"),
            CraftButton(event.recipe_id),
            Interaction::None,
        ));
    });
}
