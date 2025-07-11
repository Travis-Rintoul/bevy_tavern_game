use crate::core::InterfaceSetup;
use bevy::prelude::*;

pub fn set_interface_setup(mut interface_setup: ResMut<InterfaceSetup>) {
    interface_setup.0 = true;
}

pub fn clear_interface_setup(mut interface_setup: ResMut<InterfaceSetup>) {
    interface_setup.0 = false;
}

pub fn find_parent_with_component<T: Component>(
    mut current: Entity,
    parent_query: &Query<&ChildOf>,
    query: &Query<Entity, With<T>>,
) -> Option<Entity> {
    loop {
        if query.get(current).is_ok() {
            return Some(current);
        }

        if let Ok(parent) = parent_query.get(current) {
            current = parent.0;
        } else {
            return None;
        }
    }
}
