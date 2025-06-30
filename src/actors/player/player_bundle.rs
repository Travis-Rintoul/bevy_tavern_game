use crate::core::{Inventory, ItemID, Player};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    inventory: Inventory,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let mut inventory = Inventory::default();

        // Debug add to inventory
        inventory.add_item(ItemID::Fish, 200);

        PlayerBundle {
            marker: Player,
            inventory: inventory,
        }
    }
}
