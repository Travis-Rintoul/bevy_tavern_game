use bevy::prelude::*;

use crate::core::{DeviceType, Player, PlayerOpenedDeviceInterfaceEvent, StoveDevice, UIState};

pub fn stove_interaction(
    mut ui_state: ResMut<NextState<UIState>>,
    button: Res<ButtonInput<KeyCode>>,
    player_transform: Single<&Transform, (With<Player>, Without<StoveDevice>)>,
    stove_query: Query<(Entity, &Transform), (With<StoveDevice>, Without<Player>)>,
    mut event_writer: EventWriter<PlayerOpenedDeviceInterfaceEvent>,
) {
    if button.just_pressed(KeyCode::KeyF) {
        // TODO: remove use of single
        let Ok((stove_entity, stove_transform)) = stove_query.single() else {
            return;
        };

        if stove_transform
            .translation
            .distance(player_transform.translation)
            < 10.0
        {
            ui_state.set(UIState::DeviceStove);
            event_writer.write(PlayerOpenedDeviceInterfaceEvent {
                device: stove_entity,
                device_type: DeviceType::Stove,
            });
        }
    }
}
