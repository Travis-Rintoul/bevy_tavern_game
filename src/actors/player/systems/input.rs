use bevy::prelude::*;

use crate::core::{
    PLAYER_MOVEMENT_SPEED, Player, PlayerClosedDeviceUIEvent, PlayerClosedInventoryUIEvent,
    PlayerClosedUIEvent, PlayerInteractButtonPressedEvent, PlayerMovedEvent,
    PlayerOpenedCraftingStatusUIEvent, PlayerOpenedInventoryUIEvent, UIState,
};

pub fn on_player_movement_input(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<PlayerMovedEvent>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player_transform.translation +=
        direction.normalize_or_zero() * PLAYER_MOVEMENT_SPEED * time.delta_secs();

    event_writer.write(PlayerMovedEvent);
}

pub fn on_player_interaction_input(
    button: Res<ButtonInput<KeyCode>>,
    mut player_interact_event: EventWriter<PlayerInteractButtonPressedEvent>,
    mut inventory_open_event: EventWriter<PlayerOpenedInventoryUIEvent>,
    mut closed_ui_event: EventWriter<PlayerClosedDeviceUIEvent>,
    mut craft_status_ui_event: EventWriter<PlayerOpenedCraftingStatusUIEvent>,
) {
    if button.just_pressed(KeyCode::KeyF) {
        player_interact_event.write(PlayerInteractButtonPressedEvent);
    }

    if button.just_pressed(KeyCode::KeyI) {
        inventory_open_event.write(PlayerOpenedInventoryUIEvent);
    }

    if button.just_pressed(KeyCode::KeyC) {
        craft_status_ui_event.write(PlayerOpenedCraftingStatusUIEvent);
    }

    if button.just_pressed(KeyCode::Escape) {
        closed_ui_event.write(PlayerClosedDeviceUIEvent);
    }
}

pub fn on_player_close_ui_event(
    mut events: EventReader<PlayerClosedUIEvent>,
    ui_state: Res<State<UIState>>,
    mut close_events: (
        EventWriter<PlayerClosedDeviceUIEvent>,
        EventWriter<PlayerClosedInventoryUIEvent>,
    ),
) {
    for _ in events.read() {
        match ui_state.get() {
            UIState::DeviceStove => {
                close_events.0.write(PlayerClosedDeviceUIEvent);
            }
            UIState::Inventory => {
                close_events.1.write(PlayerClosedInventoryUIEvent);
            }
            _ => panic!("Unhandled UI State close"),
        };
    }
}
