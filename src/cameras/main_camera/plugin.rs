use bevy::prelude::*;

use crate::{cameras::main_camera::systems::player_follow, core::player_exists};

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_follow.run_if(player_exists));
    }
}
