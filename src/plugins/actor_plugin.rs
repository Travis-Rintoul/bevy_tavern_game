use bevy::prelude::*;

use crate::actors::{ActorPlayerPlugin, CustomerActorPlugin};

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActorPlayerPlugin)
            .add_plugins(CustomerActorPlugin);
    }
}
