use bevy::prelude::*;

use crate::scenes::TestScenePlugin;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TestScenePlugin);
    }
}
