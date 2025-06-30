use bevy::prelude::*;

use crate::core::{
    INVENTORY_WINDOW_GRID_SIZE, INVENTORY_WINDOW_GRID_TILE_SIZE,
    INVENTORY_WINDOW_GRID_TILE_SPACING, InventoryWindow, Owner,
};
#[derive(Bundle)]
pub struct InventoryWindowBundle {
    marker: InventoryWindow,
    owner: Owner,
    node: Node,
    bg_color: BackgroundColor,
}

impl Default for InventoryWindowBundle {
    fn default() -> Self {
        InventoryWindowBundle {
            marker: InventoryWindow,
            owner: Owner::default(),
            node: Node {
                position_type: PositionType::Relative,
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![
                    GridTrack::px(INVENTORY_WINDOW_GRID_TILE_SIZE);
                    INVENTORY_WINDOW_GRID_SIZE
                ],
                grid_template_rows: vec![
                    GridTrack::px(INVENTORY_WINDOW_GRID_TILE_SIZE);
                    INVENTORY_WINDOW_GRID_SIZE
                ],
                row_gap: Val::Px(INVENTORY_WINDOW_GRID_TILE_SPACING),
                column_gap: Val::Px(INVENTORY_WINDOW_GRID_TILE_SPACING),
                padding: UiRect {
                    left: Val::Px(25.0),
                    right: Val::Px(25.0),
                    top: Val::Px(25.0),
                    bottom: Val::Px(25.0),
                },
                ..Default::default()
            },
            bg_color: BackgroundColor(Color::srgb_u8(255, 255, 255)),
        }
    }
}

impl InventoryWindowBundle {
    pub fn new(owner: Owner) -> Self {
        let mut default = InventoryWindowBundle::default();
        default.owner = owner;
        default
    }
}
