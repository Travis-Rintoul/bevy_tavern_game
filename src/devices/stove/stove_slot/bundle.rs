use bevy::prelude::*;

use crate::core::StoveSlot;

#[derive(Bundle)]
pub struct StoveSlotBundle {
    marker: StoveSlot,
}

impl Default for StoveSlotBundle {
    fn default() -> Self {
        StoveSlotBundle {
            marker: StoveSlot::default(),
        }
    }
}

impl StoveSlotBundle {
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((self));
    }
}
