use bevy::prelude::*;

use crate::core::PlayerOpenedInventoryUIEvent;

pub fn player_handle_ui_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: (EventWriter<PlayerOpenedInventoryUIEvent>,),
) {
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        events.0.write(PlayerOpenedInventoryUIEvent);
    }
}
