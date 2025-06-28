use once_cell::sync::Lazy;

use crate::core::{ItemID, Recipe, RecipeID};

pub const PLAYER_MOVEMENT_SPEED: f32 = 6.0;

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
