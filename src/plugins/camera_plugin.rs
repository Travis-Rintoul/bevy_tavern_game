use bevy::prelude::*;

use crate::cameras::MainCameraPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainCameraPlugin);
    }
}
