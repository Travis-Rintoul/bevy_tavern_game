use bevy::prelude::*;

use crate::core::{CraftingQueue, CraftingStation, Device, DeviceType, Interactable, StoveDevice};

#[derive(Bundle)]
pub struct StoveDeviceBundle {
    device_marker: Device,
    stove_marker: StoveDevice,
    station: CraftingStation,
    crafting: CraftingQueue,
    interactable: Interactable,
}

impl Default for StoveDeviceBundle {
    fn default() -> Self {
        StoveDeviceBundle {
            device_marker: Device::default(),
            stove_marker: StoveDevice::default(),
            station: CraftingStation::default(),
            crafting: CraftingQueue::default(),
            interactable: Interactable::device(DeviceType::Stove),
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
