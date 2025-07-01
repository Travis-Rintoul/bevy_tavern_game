use crate::core::InterfaceSetup;
use bevy::prelude::*;

pub fn set_interface_setup(mut interface_setup: ResMut<InterfaceSetup>) {
    interface_setup.0 = true;
}

pub fn clear_interface_setup(mut interface_setup: ResMut<InterfaceSetup>) {
    interface_setup.0 = false;
}
