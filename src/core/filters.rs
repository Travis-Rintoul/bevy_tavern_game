use bevy::prelude::*;

use crate::core::Player;

pub fn player_exists(query: Query<(), With<Player>>) -> bool {
    return !query.is_empty();
}
