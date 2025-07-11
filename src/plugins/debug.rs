use bevy::prelude::*;

use crate::core::{CraftingQueue, CraftingStation, PlayerOpenedDeviceUIEvent};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen)
            .add_systems(Update, listen_craft);
    }
}

fn listen(mut events: EventReader<PlayerOpenedDeviceUIEvent>) {
    for e in events.read() {
        println!(
            "Player Opened device interface {} {:?}",
            e.device, e.device_type
        );
    }
}

fn listen_craft(query: Query<&CraftingQueue, Changed<CraftingStation>>) {
    for crafting in query.iter() {
        if crafting.queue.len() == 0 {
            continue;
        }

        let task = &crafting.queue[0];
        println!(
            "Crafting {:?} => {} of remaining {}",
            task.recipe_id, crafting.current_progress, task.time_required
        );
    }
}
