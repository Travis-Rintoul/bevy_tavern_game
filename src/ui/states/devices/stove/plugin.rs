use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

use crate::{core::UIState, ui::states::devices::stove::systems::setup};

pub struct StoveDeviceUIPlugin;

impl Plugin for StoveDeviceUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::DeviceStove), setup)
            .add_systems(
                OnExit(UIState::DeviceStove),
                clear_state_scoped_entities::<UIState>,
            );
    }
}
