use bevy::prelude::*;

use crate::core::{
    CraftButton, Crafting, CraftingStation, CraftingStationStartCraftingRequest, CraftingTask,
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, emit_crafting_requests)
            .add_systems(Update, handle_craft_requests);
    }
}

// listens for clicks on the CraftButton
fn emit_crafting_requests(
    mut query: Query<(&Interaction, &CraftButton, Entity), Changed<Interaction>>,
    parent_query: Query<&ChildOf>,
    station_query: Query<Entity, With<CraftingStation>>,
    mut writer: EventWriter<CraftingStationStartCraftingRequest>,
) {
    for (interaction, button, entity) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(station_entity) =
                find_parent_with_station(entity, &parent_query, &station_query)
            {
                writer.write(CraftingStationStartCraftingRequest {
                    recipe_id: button.0,
                    device_entity: station_entity,
                });
            }
        }
    }
}

fn handle_craft_requests(
    mut query: Query<(&mut Crafting, &mut CraftingStation)>,
    mut events: EventReader<CraftingStationStartCraftingRequest>,
) {
    for event in events.read() {
        // do crafting requests
        if let Ok((mut crafting, mut station)) = query.get_mut(event.device_entity) {
            crafting.paused = false;
            station.queue.push(CraftingTask {
                recipe_id: event.recipe_id,
                time_required: 10.0,
            });
        }
    }
}

fn action_opon_crafting_queue(query: Query<(&mut Crafting, &mut CraftingStation)>) {}

fn find_parent_with_station(
    mut current: Entity,
    parent_query: &Query<&ChildOf>,
    station_query: &Query<Entity, With<CraftingStation>>,
) -> Option<Entity> {
    loop {
        if station_query.get(current).is_ok() {
            return Some(current);
        }

        if let Ok(parent) = parent_query.get(current) {
            current = parent.0;
        } else {
            return None;
        }
    }
}
