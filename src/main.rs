use bevy::prelude::*;

use crate::plugins::{
    ActorPlugin, CameraPlugin, CorePlugin, DebugPlugin, InventoryPlugin, SceneManagerPlugin,
    UIPlugin,
};

mod actors;
mod cameras;
mod core;
mod devices;
mod plugins;
mod scenes;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CorePlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(InventoryPlugin)
        .run();
}
