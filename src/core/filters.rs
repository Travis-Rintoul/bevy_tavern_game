use bevy::prelude::*;

use crate::core::{InterfaceSetup, Player, UIState};

pub fn player_exists(query: Query<(), With<Player>>) -> bool {
    return !query.is_empty();
}

fn in_menu_state(ui_state: Res<State<UIState>>) -> bool {
    let menu_states: Vec<UIState> = vec![UIState::DeviceStove, UIState::Dialog, UIState::Inventory];
    menu_states.contains(ui_state.get())
}

pub fn interface_not_setup(interface_setup: Res<InterfaceSetup>) -> bool {
    !interface_setup.0
}
