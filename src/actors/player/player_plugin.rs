use bevy::prelude::*;

use crate::{
    actors::player::systems::{
        on_player_close_ui_event, on_player_interaction_event, on_player_interaction_input,
        on_player_movement_input, on_player_proximity_change,
    },
    core::InterfaceFlowSet,
};

pub struct ActorPlayerPlugin;

impl Plugin for ActorPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_player_movement_input);
        app.add_systems(PreUpdate, on_player_proximity_change);
        app.add_systems(PreUpdate, on_player_close_ui_event);
        app.add_systems(
            PreUpdate,
            on_player_interaction_event.in_set(InterfaceFlowSet::InputBufferHook),
        );
        app.add_systems(
            PreUpdate,
            on_player_interaction_input.in_set(InterfaceFlowSet::InputHook),
        );
    }
}
