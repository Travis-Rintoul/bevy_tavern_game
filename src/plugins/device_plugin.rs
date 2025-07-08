use bevy::prelude::*;

use crate::{
    core::{ActiveDeviceResource, InterfaceFlowSet, PlayerOpenedDeviceInterfaceEvent},
    devices::StoveDevicePlugin,
};

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StoveDevicePlugin);
        app.add_systems(
            PreUpdate,
            update_active_device_resource.in_set(InterfaceFlowSet::EntryHook),
        );
    }
}

fn update_active_device_resource(
    mut active_device: ResMut<ActiveDeviceResource>,
    mut events: EventReader<PlayerOpenedDeviceInterfaceEvent>,
) {
    for event in events.read() {
        active_device.0 = Some(event.device);
    }
}
