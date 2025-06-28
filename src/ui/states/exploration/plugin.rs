use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

use crate::{core::UIState, ui::states::exploration::systems::setup};

pub struct ExplorationUIPlugin;

impl Plugin for ExplorationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Exploration), setup)
            .add_systems(
                OnExit(UIState::Exploration),
                clear_state_scoped_entities::<UIState>,
            );
    }
}
