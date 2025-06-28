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
