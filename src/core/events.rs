use bevy::prelude::*;

use crate::core::{DeviceType, RecipeID};

#[derive(Event)]
pub struct PlayerMovedEvent;

#[derive(Event)]
pub struct PlayerOpenedDeviceInterfaceEvent {
    pub device: Entity,
    pub device_type: DeviceType,
}

#[derive(Event)]
pub struct PlayerClosedDeviceInterfaceEvent;

#[derive(Event)]
pub struct PlayerOpenInventoryScreenEvent;

#[derive(Event)]
pub struct PlayerClosedInventoryScreenEvent;

#[derive(Event)]
pub struct InventoryWindowPopulationRequestEvent {
    pub window_entity: Entity,
    pub inventory_entity: Entity,
}

#[derive(Event)]
pub struct RecipeListWindowPopulationRequestEvent {
    pub inventory_entity: Entity,
    pub device_type: DeviceType,
}

#[derive(Event)]
pub struct RecipeWindowPopulationRequestEvent {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct RecipeListWindowOptionSelected {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct CraftingStationStartCraftingRequest {
    pub recipe_id: RecipeID,
}

#[derive(Event)]
pub struct CraftingStationFinishedCraftingRequest {
    pub recipe_id: RecipeID,
    pub station_entity: Entity,
}
