use bevy::prelude::*;

use crate::core::Player;

pub fn player_follow(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Some(player_transform) = player_query.single().ok() else {
        return;
    };

    let Some(mut camera_transform) = camera_query.single_mut().ok() else {
        return;
    };

    // Desired offset in world space
    let offset = Vec3::new(10.0, 12.0, 10.0);

    // Update camera position and rotation
    camera_transform.translation = player_transform.translation() + offset;
    camera_transform.look_at(player_transform.translation(), Vec3::Y);
}
