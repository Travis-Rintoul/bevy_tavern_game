use bevy::prelude::*;

use crate::ui::{
    InventoryItemWindowUIElementPlugin, InventoryWindowUIElementPlugin,
    RecipeListWindowUIElementPlugin, RecipeWindowUIElementPlugin,
};

pub struct UIElementsPlugin;

impl Plugin for UIElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InventoryWindowUIElementPlugin)
            .add_plugins(InventoryItemWindowUIElementPlugin)
            .add_plugins(RecipeListWindowUIElementPlugin)
            .add_plugins(RecipeWindowUIElementPlugin);
    }
}
