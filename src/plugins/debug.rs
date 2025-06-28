use bevy::prelude::*;

use crate::core::PlayerOpenedDeviceInterfaceEvent;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen);
    }
}

fn listen(mut events: EventReader<PlayerOpenedDeviceInterfaceEvent>) {
    for e in events.read() {
        println!(
            "Player Opened device interface {} {:?}",
            e.device, e.device_type
        );
    }
}
