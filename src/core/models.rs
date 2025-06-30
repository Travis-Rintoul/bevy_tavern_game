use crate::core::{ItemID, RecipeID};

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: RecipeID,
    pub name: &'static str,
    pub required_items: Vec<ItemID>,
    pub cook_time: f32, // Seconds
    pub output_item: ItemID,
}

#[derive(Debug, Clone)]
pub struct InventoryItemStack {
    pub item_id: ItemID,
    pub item_count: u32,
    pub max_size: u32,
}

impl InventoryItemStack {
    pub fn new(item_id: ItemID, item_count: u32) -> Self {
        InventoryItemStack {
            item_id: item_id,
            item_count: item_count,
            max_size: InventoryItemStack::get_stack_size(item_id),
        }
    }

    fn get_stack_size(item_id: ItemID) -> u32 {
        128
    }
}
