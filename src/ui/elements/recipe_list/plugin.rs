use bevy::prelude::*;

use crate::ui::elements::recipe_list::systems::populate_recipe_window;

pub struct RecipeListWindowUIElementPlugin;

impl Plugin for RecipeListWindowUIElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_recipe_window);
    }
}
