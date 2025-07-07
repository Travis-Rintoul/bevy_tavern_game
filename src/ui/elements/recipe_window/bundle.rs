use bevy::prelude::*;

use crate::core::RecipeWindow;

#[derive(Bundle)]
pub struct RecipeWindowBundle {
    marker: RecipeWindow,
}

impl Default for RecipeWindowBundle {
    fn default() -> Self {
        RecipeWindowBundle {
            marker: RecipeWindow,
        }
    }
}
