use bevy::{ecs::event, prelude::*};

use crate::core::{ALL_RECIPES, CraftingFinishedEvent, Inventory, InventoryItemAddedEvent};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {}
}

fn on_add_inventory_item_event(
    mut events: EventReader<InventoryItemAddedEvent>,
    inventory_query: Query<&mut Inventory>,
) {
}

fn on_remove_inventory_item_event() {}

fn on_transfer_inventory_item_event() {}
