use crate::core::ItemID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipeID {
    CookedMeat,
    VegetableSoup,
    GrilledFish,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: RecipeID,
    pub name: &'static str,
    pub required_items: Vec<ItemID>,
    pub cook_time: f32, // Seconds
    pub output_item: ItemID,
}
