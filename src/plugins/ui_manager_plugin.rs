use bevy::prelude::*;

use crate::{
    core::{
        PLAYER_DEFAULT_UI_STATE, PlayerClosedDeviceInterfaceEvent,
        PlayerClosedInventoryScreenEvent, PlayerOpenInventoryScreenEvent, UIState,
    },
    plugins::UIElementsPlugin,
    ui::{ExplorationUIPlugin, InventroyUIPlugin, StoveDeviceUIPlugin},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIElementsPlugin);

        // add ui states
        app.add_plugins(ExplorationUIPlugin)
            .add_plugins(StoveDeviceUIPlugin)
            .add_plugins(InventroyUIPlugin);

        // add common ui states
        app.add_systems(
            Update,
            (
                handle_device_closed_event,
                handle_inventory_opened_event,
                handle_inventory_closed_event,
            ),
        );
    }
}

fn handle_device_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedDeviceInterfaceEvent>,
) {
    for _ in events.read() {
        ui_state.set(PLAYER_DEFAULT_UI_STATE)
    }
}

fn handle_inventory_opened_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerOpenInventoryScreenEvent>,
) {
    for _ in events.read() {
        ui_state.set(UIState::Inventory)
    }
}

fn handle_inventory_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedInventoryScreenEvent>,
) {
    for _ in events.read() {
        ui_state.set(PLAYER_DEFAULT_UI_STATE)
    }
}
