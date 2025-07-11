use bevy::prelude::*;

use crate::core::{PlayerClosedDeviceUIEvent, PlayerClosedInventoryUIEvent, UIState};

pub fn close_menus(
    ui_state: Res<State<UIState>>,
    button: Res<ButtonInput<KeyCode>>,
    mut events: (
        EventWriter<PlayerClosedDeviceUIEvent>,
        EventWriter<PlayerClosedInventoryUIEvent>,
    ),
) {
    if button.just_pressed(KeyCode::Escape) {
        match ui_state.get() {
            UIState::DeviceStove => {
                events.0.write(PlayerClosedDeviceUIEvent);
            }
            UIState::Inventory => {
                events.1.write(PlayerClosedInventoryUIEvent);
            }
            _ => panic!("Unhandled UI State close"),
        };
    }
}
