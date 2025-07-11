use bevy::{
    ecs::{event, query::QuerySingleError},
    prelude::*,
};
use bevy_inspector_egui::inspector_options::Target;

use crate::core::{
    ALL_RECIPES, CraftingFinishedEvent, Inventory, InventoryItemAddedEvent,
    InventoryItemRemovedEvent, InventoryItemTransferredEvent,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_add_inventory_item_event);
        app.add_systems(PreUpdate, on_remove_inventory_item_event);
        app.add_systems(PreUpdate, on_transfer_inventory_item_event);
    }
}

fn on_add_inventory_item_event(
    mut events: EventReader<InventoryItemAddedEvent>,
    mut inventory_query: Query<&mut Inventory>,
) {
    for event in events.read() {
        match inventory_query.get_mut(event.target_entity) {
            Err(_) => error!(
                "Failed to add item to inventory: Target entity {:?} does not have an Inventory component.",
                event.target_entity
            ),
            Ok(mut inventory) => {
                inventory.add_item(event.item_id, event.quantity);
            }
        }
    }
}

fn on_remove_inventory_item_event(
    mut events: EventReader<InventoryItemRemovedEvent>,
    mut inventory_query: Query<&mut Inventory>,
) {
    for event in events.read() {
        match inventory_query.get_mut(event.target_entity) {
            Err(_) => error!(
                "Failed to remove item from inventory: Target entity {:?} does not have an Inventory component.",
                event.target_entity
            ),
            Ok(mut inventory) => {
                inventory.remove_item(event.item_id, event.quantity);
            }
        }
    }
}

fn on_transfer_inventory_item_event(
    mut events: EventReader<InventoryItemTransferredEvent>,
    mut inventory_query: Query<&mut Inventory>,
) {
    for event in events.read() {
        match inventory_query.get_many_mut([event.sender_entity, event.target_entity]) {
            Err(_) => {
                error!(
                    "Unable to fetch sender inventory or receiver entity via ({}, {}) respectively",
                    event.sender_entity, event.target_entity
                );
            }
            Ok([mut sender_inventory, mut target_inventory]) => {
                sender_inventory.remove_item(event.item_id, event.quantity);
                target_inventory.add_item(event.item_id, event.quantity);
            }
        }
    }
}
