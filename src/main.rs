//#![allow(dead_code, unused)]

use crate::plugins::{
    ActorPlugin, CameraPlugin, CorePlugin, CraftingPlugin, DebugPlugin, DevicePlugin,
    InventoryPlugin, RecipePlugin, SceneManagerPlugin, UIPlugin,
};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use std::{env, io::Write};

mod actors;
mod cameras;
mod core;
mod devices;
mod plugins;
mod scenes;
mod ui;

fn main() {
    // clean up bevy logs
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "off,bevy_tavern_game=debug") };
    }

    // Overwrite env_logger default
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        // .add_plugins(EguiPlugin {
        //     enable_multipass_for_primary_context: true,
        // })
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DevicePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(RecipePlugin)
        .add_plugins(CraftingPlugin)
        .run();
}
