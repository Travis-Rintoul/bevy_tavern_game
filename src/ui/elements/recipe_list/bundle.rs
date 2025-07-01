use bevy::prelude::*;

use crate::core::RecipeListWindow;
#[derive(Bundle)]
pub struct RecipeListWindowBundle {
    marker: RecipeListWindow,
}

impl Default for RecipeListWindowBundle {
    fn default() -> Self {
        RecipeListWindowBundle {
            marker: RecipeListWindow,
        }
    }
}
