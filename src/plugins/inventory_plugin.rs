use bevy::prelude::*;

use crate::core::{
    InventoryWindow, InventoryWindowPopulationRequestEvent, Owner, Player,
    PlayerOpenInventoryScreenEvent,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, emit_inventory_window_population_request);
    }
}

fn emit_inventory_window_population_request(
    player_query: Query<Entity, With<Player>>,
    events: EventReader<PlayerOpenInventoryScreenEvent>,
    inventory_window_query: Query<(Entity, Option<&Owner>), With<InventoryWindow>>,
    mut population_request_event: EventWriter<InventoryWindowPopulationRequestEvent>,
) {
    if !events.is_empty() {
        let Ok(player_entity) = player_query.single() else {
            // TODO: make message a constant
            panic!("Unable to find player");
        };

        for (window_entity, owner) in &inventory_window_query {
            if matches!(owner, Some(Owner::Player)) {
                // Tell the UI to display the ui items
                population_request_event.write(InventoryWindowPopulationRequestEvent {
                    window_entity: window_entity,
                    inventory_entity: player_entity,
                });
            }
        }
    }
}
