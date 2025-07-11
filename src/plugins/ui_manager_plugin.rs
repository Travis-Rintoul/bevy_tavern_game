use bevy::prelude::*;

use crate::{
    core::{
        DeviceType, InterfaceFlowSet, PLAYER_DEFAULT_UI_STATE, PlayerClosedCraftingStatusUIEvent,
        PlayerClosedDeviceUIEvent, PlayerClosedInventoryUIEvent, PlayerOpenedCraftingStatusUIEvent,
        PlayerOpenedDeviceUIEvent, PlayerOpenedInventoryUIEvent, UIState,
    },
    plugins::UIElementsPlugin,
    ui::{CraftingStatusUIPlugin, ExplorationUIPlugin, InventroyUIPlugin, StoveDeviceUIPlugin},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIElementsPlugin);

        // add ui states
        app.add_plugins(ExplorationUIPlugin)
            .add_plugins(StoveDeviceUIPlugin)
            .add_plugins(InventroyUIPlugin)
            .add_plugins(CraftingStatusUIPlugin);

        // add common ui states
        app.add_systems(
            PreUpdate,
            (
                handle_device_open_event.in_set(InterfaceFlowSet::EntryHook),
                handle_device_closed_event,
                handle_inventory_opened_event.in_set(InterfaceFlowSet::EntryHook),
                handle_inventory_closed_event,
                handle_craft_status_opened_event.in_set(InterfaceFlowSet::EntryHook),
                handle_craft_status_closed_event,
            ),
        );
    }
}

fn handle_device_open_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerOpenedDeviceUIEvent>,
) {
    for event in events.read() {
        match event.device_type {
            DeviceType::Stove => ui_state.set(UIState::DeviceStove),
        }
    }
}

fn handle_device_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedDeviceUIEvent>,
) {
    for _ in events.read() {
        ui_state.set(PLAYER_DEFAULT_UI_STATE)
    }
}

fn handle_inventory_opened_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerOpenedInventoryUIEvent>,
) {
    for _ in events.read() {
        ui_state.set(UIState::Inventory)
    }
}

fn handle_inventory_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedInventoryUIEvent>,
) {
    for _ in events.read() {
        ui_state.set(PLAYER_DEFAULT_UI_STATE)
    }
}

fn handle_craft_status_opened_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerOpenedCraftingStatusUIEvent>,
) {
    for _ in events.read() {
        ui_state.set(UIState::CraftingStatus)
    }
}

fn handle_craft_status_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedCraftingStatusUIEvent>,
) {
    for _ in events.read() {
        ui_state.set(PLAYER_DEFAULT_UI_STATE)
    }
}
