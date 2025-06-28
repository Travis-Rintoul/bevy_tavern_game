use crate::core::Player;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle { marker: Player }
    }
}
