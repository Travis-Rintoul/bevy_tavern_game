use bevy::{ecs::event, prelude::*};

use crate::core::{ALL_RECIPES, CraftingStationFinishedCraftingRequest, Inventory};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_craft_task_finish);
    }
}

// Add item to inventory of crafting station
fn handle_craft_task_finish(
    mut inventory_query: Query<&mut Inventory>,
    mut events: EventReader<CraftingStationFinishedCraftingRequest>,
) {
    for event in events.read() {
        let Ok(mut inventory) = inventory_query.get_mut(event.station_entity) else {
            continue;
        };

        // TODO: implement better recipe fetch
        let Some(recipe) = ALL_RECIPES
            .iter()
            .find(|predicate| predicate.id == event.recipe_id)
        else {
            // TODO: handle error
            panic!("recipe not found");
        };

        inventory.add_item(recipe.output_item, 1);

        println!(
            "Added {} ({}) to {}'s inventory",
            recipe.output_item, 1, event.station_entity
        );
    }
}
