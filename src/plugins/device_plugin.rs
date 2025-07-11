use bevy::{prelude::*, state::commands};

use crate::{
    core::{
        ActiveDeviceResource, InterfaceFlowSet, Owner, Player, PlayerOpenedDeviceUIEvent,
        RecipeListWindow, RequestRecipeListUIPopulationEvent,
    },
    devices::StoveDevicePlugin,
};

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StoveDevicePlugin);
        app.add_systems(
            PreUpdate,
            (
                update_active_device_resource.in_set(InterfaceFlowSet::EntryHook),
                on_player_opened_device.in_set(InterfaceFlowSet::AfterHook),
            ),
        );
    }
}

fn update_active_device_resource(
    mut events: EventReader<PlayerOpenedDeviceUIEvent>,
    mut active_device: ResMut<ActiveDeviceResource>,
) {
    for event in events.read() {
        active_device.0 = Some(event.device);
    }
}

fn on_player_opened_device(
    mut events: EventReader<PlayerOpenedDeviceUIEvent>,
    mut active_device: ResMut<ActiveDeviceResource>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    recipe_window_query: Query<(Entity, Option<&Owner>), With<RecipeListWindow>>,
) {
    for event in events.read() {
        let Ok(player_entity) = player_query.single() else {
            return error!("Unable to find player in world");
        };

        if !event.needs_recipes {
            return;
        }

        active_device.0 = Some(event.device);

        for (window_entity, owner) in &recipe_window_query {
            let Some(Owner::Device(device_entity)) = owner else {
                continue;
            };

            if *device_entity == event.device {
                commands
                    .entity(window_entity)
                    .trigger(RequestRecipeListUIPopulationEvent {
                        inventory_entity: player_entity,
                        device_type: event.device_type.clone(),
                    });
            }
        }
    }
}
