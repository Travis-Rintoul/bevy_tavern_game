use bevy::prelude::*;

use crate::core::{PLAYER_MOVEMENT_SPEED, Player, PlayerMovedEvent};

pub fn player_movement(
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

    event_writer.write(PlayerMovedEvent);

    player_transform.translation +=
        direction.normalize_or_zero() * PLAYER_MOVEMENT_SPEED * time.delta_secs();
}
