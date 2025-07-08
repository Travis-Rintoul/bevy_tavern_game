use bevy::prelude::*;

use crate::{
    actors::player::systems::{
        close_menus, player_handle_ui_input, player_movement, stove_interaction,
    },
    core::{InterfaceFlowSet, player_exists},
};

pub struct ActorPlayerPlugin;

impl Plugin for ActorPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (stove_interaction, player_handle_ui_input).in_set(InterfaceFlowSet::InputHook),
        );

        app.add_systems(Update, (player_movement, close_menus).run_if(player_exists));
    }
}
