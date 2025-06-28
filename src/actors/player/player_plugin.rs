use bevy::prelude::*;

use crate::{
    actors::player::systems::{player_movement, stove_interaction},
    core::player_exists,
};

pub struct ActorPlayerPlugin;

impl Plugin for ActorPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, stove_interaction).run_if(player_exists),
        );
    }
}
