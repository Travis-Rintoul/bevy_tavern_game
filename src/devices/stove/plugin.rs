use bevy::prelude::*;

use crate::devices::stove::stove_slot::StoveSlotPlugin;

pub struct StoveDevicePlugin;

impl Plugin for StoveDevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StoveSlotPlugin);
    }
}
