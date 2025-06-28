use bevy::prelude::*;

mod devices;
mod player;
mod plugins;
mod scenes;
mod ui;

fn main() {
    App::new().add_plugins(DefaultPlugins);
}
