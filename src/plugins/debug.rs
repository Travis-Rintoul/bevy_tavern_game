use bevy::prelude::*;

use crate::core::{Crafting, CraftingStation, PlayerOpenedDeviceInterfaceEvent};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen)
            .add_systems(Update, listen_craft);
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

fn listen_craft(query: Query<(&Crafting, &CraftingStation)>) {
    for (crafting, station) in query.iter() {
        if !crafting.0 {
            continue;
        }

        println!("{:?}", station.queue);
    }
}
