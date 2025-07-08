use bevy::prelude::*;

use crate::ui::elements::inventory_window::systems::populate_inventory_window;

pub struct InventoryWindowUIElementPlugin;

impl Plugin for InventoryWindowUIElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(populate_inventory_window);
    }
}
