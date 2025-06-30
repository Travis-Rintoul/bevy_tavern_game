use bevy::prelude::*;

use crate::{
    actors::player::systems::{
        close_menus, player_handle_ui_input, player_movement, stove_interaction,
    },
    core::player_exists,
};

pub struct ActorPlayerPlugin;

impl Plugin for ActorPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement,
                stove_interaction,
                player_handle_ui_input,
                close_menus,
            )
                .run_if(player_exists),
        );
    }
}
