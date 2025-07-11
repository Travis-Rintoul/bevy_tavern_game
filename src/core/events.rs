use bevy::prelude::*;

use crate::core::{DeviceType, InventoryAddReason, ItemID, RecipeID};

#[derive(Event)]
pub struct PlayerMovedEvent;

#[derive(Event)]
pub struct PlayerOpenedDeviceUIEvent {
    pub device: Entity,
    pub device_type: DeviceType,
}

#[derive(Event)]
pub struct PlayerClosedDeviceUIEvent;

#[derive(Event)]
pub struct PlayerOpenedInventoryUIEvent;

#[derive(Event)]
pub struct PlayerClosedInventoryUIEvent;

#[derive(Event)]
pub struct RequestInventoryUIPopulationEvent {
    pub window_entity: Entity,
    pub inventory_entity: Entity,
}

#[derive(Event)]
pub struct RequestRecipeListUIPopulationEvent {
    pub inventory_entity: Entity,
    pub device_type: DeviceType,
}

#[derive(Event)]
pub struct RequestRecipeUIPopulationEvent {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct RecipeListOptionSelectedEvent {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct StartCraftingRequestEvent {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct CraftingFinishedEvent {
    pub recipe_id: RecipeID,
    pub station_entity: Entity,
}

#[derive(Event)]
pub struct InventoryItemAddedEvent {
    pub item_id: ItemID,
    pub quantity: u32,
    pub reason: InventoryAddReason,
}

// Event when an item is removed from inventory
struct InventoryItemRemovedEvent {
    // fields like item ID, quantity, etc.
}

// Event when an item is transferred between inventories
struct InventoryItemTransferredEvent {
    // fields like source inventory, destination inventory, item ID, quantity, etc.
}
