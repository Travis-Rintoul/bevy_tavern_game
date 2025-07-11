use bevy::prelude::*;

use crate::core::InteractableType;

#[derive(Resource, Default)]
pub struct ActiveDeviceResource(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct InterfaceSetup(pub bool);

#[derive(Resource)]
pub struct ActiveInteractable {
    pub entity: Entity,
    pub interactable_type: InteractableType,
}
