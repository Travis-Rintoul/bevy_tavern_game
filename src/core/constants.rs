use once_cell::sync::Lazy;

use crate::core::{ItemID, Recipe, RecipeID, UIState};

pub const PLAYER_MOVEMENT_SPEED: f32 = 6.0;
pub const PLAYER_DEFAULT_UI_STATE: UIState = UIState::Exploration;
pub const INVENTORY_WINDOW_GRID_SIZE: usize = 10;
pub const INVENTORY_WINDOW_GRID_TILE_SIZE: f32 = 100.0;
pub const INVENTORY_WINDOW_GRID_TILE_SPACING: f32 = 5.0;

pub static ALL_RECIPES: Lazy<Vec<Recipe>> = Lazy::new(|| {
    vec![
        Recipe {
            id: RecipeID::CookedMeat,
            name: "Cooked Meat",
            required_items: vec![ItemID::RawMeat],
            cook_time: 5.0,
            output_item: ItemID::CookedMeat,
        },
        Recipe {
            id: RecipeID::VegetableSoup,
            name: "Vegetable Soup",
            required_items: vec![ItemID::Herb, ItemID::Fish],
            cook_time: 8.0,
            output_item: ItemID::VegetableSoup,
        },
    ]
});
