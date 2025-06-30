use crate::core::{ItemID, RecipeID};

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: RecipeID,
    pub name: &'static str,
    pub required_items: Vec<ItemStack>,
    pub cook_time: f32, // Seconds
    pub output_item: ItemID,
}

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item_id: ItemID,
    pub item_count: u32,
}

impl ItemStack {
    pub fn new(item_id: ItemID, item_count: u32) -> Self {
        ItemStack {
            item_id: item_id,
            item_count: item_count,
        }
    }
}
