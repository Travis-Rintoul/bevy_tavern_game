use bevy::{prelude::*, render::render_resource::encase::private::RuntimeSizedArray};

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

fn listen_craft(query: Query<(&Crafting, &CraftingStation), Changed<CraftingStation>>) {
    for (crafting, station) in query.iter() {
        if station.queue.len() == 0 {
            continue;
        }

        let task = &station.queue[0];
        println!(
            "Crafting {:?} => {} of remaining {}",
            task.recipe_id, station.current_progress, task.time_required
        );
    }
}
