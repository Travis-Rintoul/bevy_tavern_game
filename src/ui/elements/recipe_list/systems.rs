use bevy::prelude::*;

use crate::core::{ALL_RECIPES, RecipeListWindow, RecipeWindowPopulationRequestEvent};

pub fn populate_recipe_window(
    mut commands: Commands,
    children_query: Query<&Children>,
    recipe_window_query: Query<&RecipeListWindow>,
    mut events: EventReader<RecipeWindowPopulationRequestEvent>,
) {
    for event in events.read() {
        let Ok(_) = recipe_window_query.get(event.window_entity) else {
            continue;
        };

        if let Ok(children) = children_query.get(event.window_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        let relavent_recipies = &ALL_RECIPES;

        commands
            .entity(event.window_entity)
            .with_children(|parent| {
                relavent_recipies.iter().for_each(|recipe| {
                    parent
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },))
                        .with_children(|recipe_record| {
                            // Spawn stack count
                            recipe_record.spawn((
                                Text::new(recipe.name),
                                TextColor(Color::srgb_u8(255, 255, 255)),
                            ));
                        });
                });
            });
    }
}
