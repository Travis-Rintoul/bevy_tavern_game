use bevy::{ecs::query, prelude::*};

use crate::core::{
    ALL_RECIPES, CraftButton, Crafting, CraftingStation, CraftingStationFinishedCraftingRequest,
    CraftingStationStartCraftingRequest, CraftingTask, Owner, Player, Station,
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_craft_requests);
        app.add_systems(Update, (emit_crafting_requests, action_upon_crafting_queue));
    }
}

// listens for clicks on the CraftButton
fn emit_crafting_requests(
    mut commands: Commands,
    mut query: Query<(&Interaction, &CraftButton, Entity), Changed<Interaction>>,
    parent_query: Query<&ChildOf>,
    station_query: Query<Entity, With<Station>>,
    owner_query: Query<&Owner>,
    mut writer: EventWriter<CraftingStationStartCraftingRequest>,
) {
    for (interaction, button, entity) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(station_entity) =
                find_parent_with_station(entity, &parent_query, &station_query)
            {
                if let Ok(Owner::Device(owner_entity)) = owner_query.get(station_entity) {
                    commands
                        .entity(*owner_entity)
                        .trigger(CraftingStationStartCraftingRequest {
                            recipe_id: button.0,
                        });
                }
            }
        }
    }
}

fn handle_craft_requests(
    mut trigger: Trigger<CraftingStationStartCraftingRequest>,
    mut query: Query<(&mut Crafting, &mut CraftingStation)>,
) {
    // do crafting requests
    if let Ok((mut crafting, mut station)) = query.get_mut(trigger.target()) {
        // TODO: add beter way of fetching recipe
        let Some(recipe) = ALL_RECIPES
            .iter()
            .find(|predicate| predicate.id == trigger.event().recipe_id)
        else {
            panic!("unable to find recipe");
        };

        crafting.paused = true;
        station.queue.push(CraftingTask {
            recipe_id: trigger.event().recipe_id,
            time_required: recipe.cook_time,
        });
    }
}

fn action_upon_crafting_queue(
    time: Res<Time>,
    mut player_query: Single<Entity, With<Player>>,
    mut query: Query<(&mut Crafting, &mut CraftingStation, Entity)>,
    mut events: EventWriter<CraftingStationFinishedCraftingRequest>,
) {
    for (mut crafting, mut station, station_entity) in &mut query {
        if station.queue.len() == 0 {
            continue;
        }

        if crafting.timer.tick(time.delta()).just_finished() {
            station.current_progress += 1.0;

            if station.current_progress > station.queue[0].time_required {
                events.write(CraftingStationFinishedCraftingRequest {
                    recipe_id: station.queue[0].recipe_id,
                    station_entity: *player_query,
                });

                station.queue.remove(0);
                station.current_progress = 0.0;

                if station.queue.len() == 0 {
                    crafting.paused = true;
                }
            }
        }
    }
}

fn find_parent_with_station(
    mut current: Entity,
    parent_query: &Query<&ChildOf>,
    station_query: &Query<Entity, With<Station>>,
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
