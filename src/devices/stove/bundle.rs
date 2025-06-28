use bevy::prelude::*;

use crate::core::{Device, StoveDevice};

#[derive(Bundle)]
pub struct StoveDeviceBundle {
    device_marker: Device,
    stove_marker: StoveDevice,
}

impl Default for StoveDeviceBundle {
    fn default() -> Self {
        StoveDeviceBundle {
            device_marker: Device::default(),
            stove_marker: StoveDevice::default(),
        }
    }
}

impl StoveDeviceBundle {
    #[allow(unused_variables, dead_code, unused_parens)]
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((self));
    }
}
