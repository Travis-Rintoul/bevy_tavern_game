use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

use crate::{core::UIState, ui::states::inventory::systems::setup};

pub struct InventroyUIPlugin;

impl Plugin for InventroyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Inventory), setup)
            .add_systems(
                OnExit(UIState::Inventory),
                clear_state_scoped_entities::<UIState>,
            );
    }
}
