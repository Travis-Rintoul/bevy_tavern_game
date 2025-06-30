use bevy::prelude::*;

use crate::core::{INVENTORY_WINDOW_GRID_TILE_SIZE, InventoryItemStack, InventoryItemWindow};
#[derive(Bundle)]
pub struct InventoryItemWindowBundle {
    marker: InventoryItemWindow,
    node: Node,
    bg_color: BackgroundColor,
    text: Text,
}

impl Default for InventoryItemWindowBundle {
    fn default() -> Self {
        InventoryItemWindowBundle {
            marker: InventoryItemWindow::default(),
            node: Node {
                display: Display::Flex,
                width: Val::Px(INVENTORY_WINDOW_GRID_TILE_SIZE),
                height: Val::Px(INVENTORY_WINDOW_GRID_TILE_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            bg_color: BackgroundColor(Color::srgb_u8(100, 100, 100)),
            text: Text::new(""),
        }
    }
}

impl InventoryItemWindowBundle {
    pub fn from_stack(stack: &InventoryItemStack) -> impl Bundle {
        let mut default = InventoryItemWindowBundle::default();
        default.text = Text::new(stack.item_id.to_string());
        default.marker = InventoryItemWindow {
            item_id: stack.item_id,
            item_count: stack.item_count,
        };
        default
    }
}
