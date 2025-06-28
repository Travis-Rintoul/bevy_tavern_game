use bevy::prelude::*;

use crate::actors::ActorPlayerPlugin;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActorPlayerPlugin);
    }
}
