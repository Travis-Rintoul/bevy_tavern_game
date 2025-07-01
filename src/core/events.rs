use bevy::prelude::*;

use crate::core::DeviceType;

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
pub struct RecipeWindowPopulationRequestEvent {
    pub window_entity: Entity,
    pub inventory_entity: Entity,
    pub device_type: DeviceType,
}
