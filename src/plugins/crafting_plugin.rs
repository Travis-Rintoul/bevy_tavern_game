use bevy::{prelude::*, state::commands};

use crate::core::{
    ALL_RECIPES, CraftButton, CraftingFinishedEvent, CraftingQueue, CraftingStation, CraftingTask,
    Owner, Player, StartCraftingRequestEvent, Station, find_parent_with_component,
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_craft_request_event);
        app.add_systems(Update, (on_craft_button_clicked, on_crafting_queue_tick));
    }
}

fn on_craft_button_clicked(
    mut commands: Commands,
    mut query: Query<(&Interaction, &CraftButton, Entity), Changed<Interaction>>,
    query_set: (Query<&ChildOf>, Query<Entity, With<Station>>, Query<&Owner>),
) {
    for (interaction, button, entity) in query.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Some(station_entity) =
            find_parent_with_component::<Station>(entity, &query_set.0, &query_set.1)
        else {
            continue;
        };

        if let Ok(Owner::Device(owner_entity)) = query_set.2.get(station_entity) {
            commands
                .entity(*owner_entity)
                .trigger(StartCraftingRequestEvent {
                    recipe_id: button.0,
                });
        }
    }
}

fn on_craft_request_event(
    trigger: Trigger<StartCraftingRequestEvent>,
    mut query: Query<&mut CraftingQueue, With<CraftingStation>>,
) {
    let Ok(mut crafting) = query.get_mut(trigger.target()) else {
        warn!("Unable to find crafting station details and queue");
        return;
    };

    let Some(recipe) = ALL_RECIPES
        .iter()
        .find(|predicate| predicate.id == trigger.event().recipe_id)
    else {
        panic!("unable to find recipe");
    };

    crafting.paused = true;
    crafting.queue.push(CraftingTask {
        recipe_id: trigger.event().recipe_id,
        time_required: recipe.cook_time,
    });
}

fn on_crafting_queue_tick(
    time: Res<Time>,
    player_query: Single<Entity, With<Player>>,
    mut query: Query<(&mut CraftingQueue, Entity), With<CraftingStation>>,
    mut events: EventWriter<CraftingFinishedEvent>,
) {
    for (mut crafting, station_entity) in &mut query {
        if crafting.queue.len() == 0 {
            continue;
        }

        if crafting.timer.tick(time.delta()).just_finished() {
            crafting.current_progress += 1.0;

            if crafting.current_progress > crafting.queue[0].time_required {
                events.write(CraftingFinishedEvent {
                    recipe_id: crafting.queue[0].recipe_id,
                    station_entity: *player_query,
                });

                crafting.queue.remove(0);
                crafting.current_progress = 0.0;

                if crafting.queue.len() == 0 {
                    crafting.paused = true;
                }
            }
        }
    }
}
