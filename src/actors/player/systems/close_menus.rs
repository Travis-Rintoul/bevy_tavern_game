use bevy::prelude::*;

use crate::core::{PlayerClosedDeviceInterfaceEvent, PlayerClosedInventoryScreenEvent, UIState};

pub fn close_menus(
    ui_state: Res<State<UIState>>,
    button: Res<ButtonInput<KeyCode>>,
    mut events: (
        EventWriter<PlayerClosedDeviceInterfaceEvent>,
        EventWriter<PlayerClosedInventoryScreenEvent>,
    ),
) {
    if button.just_pressed(KeyCode::Escape) {
        match ui_state.get() {
            UIState::DeviceStove => {
                events.0.write(PlayerClosedDeviceInterfaceEvent);
            }
            UIState::Inventory => {
                events.1.write(PlayerClosedInventoryScreenEvent);
            }
            _ => panic!("Unhandled UI State close"),
        };
    }
}
