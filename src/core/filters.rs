use bevy::prelude::*;

use crate::core::{Player, UIState};

pub fn player_exists(query: Query<(), With<Player>>) -> bool {
    return !query.is_empty();
}

fn in_menu_state(ui_state: Res<State<UIState>>) -> bool {
    let menu_states: Vec<UIState> = vec![UIState::DeviceStove, UIState::Dialog, UIState::Inventory];
    menu_states.contains(ui_state.get())
}
