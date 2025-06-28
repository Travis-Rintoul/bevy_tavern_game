use bevy::prelude::*;

use crate::{
    core::{PlayerClosedDeviceInterfaceEvent, PlayerOpenedDeviceInterfaceEvent, UIState},
    ui::{ExplorationUIPlugin, StoveDeviceUIPlugin},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // add ui states
        app.add_plugins(ExplorationUIPlugin)
            .add_plugins(StoveDeviceUIPlugin);

        // add common ui states
        app.add_systems(
            Update,
            (
                (handle_device_opened_event, handle_device_closed_event),
                close_device_interface.run_if(in_state(UIState::DeviceStove)),
            ),
        );
    }
}

fn close_device_interface(
    button: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<PlayerClosedDeviceInterfaceEvent>,
) {
    if button.just_pressed(KeyCode::Escape) {
        writer.write(PlayerClosedDeviceInterfaceEvent);
    }
}

fn handle_device_opened_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerOpenedDeviceInterfaceEvent>,
) {
    for _ in events.read() {
        ui_state.set(UIState::DeviceStove);
    }
}

fn handle_device_closed_event(
    mut ui_state: ResMut<NextState<UIState>>,
    mut events: EventReader<PlayerClosedDeviceInterfaceEvent>,
) {
    for _ in events.read() {
        ui_state.set(UIState::Exploration)
    }
}
