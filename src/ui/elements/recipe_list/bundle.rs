use bevy::prelude::*;

use crate::core::RecipeList;
#[derive(Bundle)]
pub struct RecipeListWindowBundle {
    marker: RecipeList,
}

impl Default for RecipeListWindowBundle {
    fn default() -> Self {
        RecipeListWindowBundle { marker: RecipeList }
    }
}
