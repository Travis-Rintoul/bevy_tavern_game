use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ActiveDeviceResource(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct InterfaceSetup(pub bool);
