use bevy::prelude::*;

use crate::core::{Device, StoveSlot};

#[derive(Bundle)]
pub struct StoveSlotBundle {
    device_marker: Device,
    marker: StoveSlot,
}

impl Default for StoveSlotBundle {
    fn default() -> Self {
        StoveSlotBundle {
            device_marker: Device::default(),
            marker: StoveSlot::default(),
        }
    }
}

impl StoveSlotBundle {
    pub fn spawn(self, commands: &mut Commands) {
        commands.spawn(self);
    }
}
