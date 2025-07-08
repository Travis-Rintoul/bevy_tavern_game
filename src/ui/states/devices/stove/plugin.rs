use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

use crate::{
    core::{InterfaceFlowSet, InterfaceSetup, UIState, clear_interface_setup, set_interface_setup},
    ui::states::devices::stove::systems::setup,
};

pub struct StoveDeviceUIPlugin;

impl Plugin for StoveDeviceUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_interface_setup, setup)
                .chain()
                .run_if(condtion)
                .in_set(InterfaceFlowSet::ActionHook),
        )
        .add_systems(
            OnExit(UIState::DeviceStove),
            (
                clear_state_scoped_entities::<UIState>,
                clear_interface_setup,
            ),
        );
    }
}

fn condtion(ui_state: Res<State<UIState>>, interface_setup: Res<InterfaceSetup>) -> bool {
    *ui_state.get() == UIState::DeviceStove && !interface_setup.0
}
