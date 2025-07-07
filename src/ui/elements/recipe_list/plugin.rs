use bevy::prelude::*;

use crate::ui::elements::recipe_list::systems::{
    option_selected_observer, populate_recipe_list_window,
};

pub struct RecipeListWindowUIElementPlugin;

impl Plugin for RecipeListWindowUIElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(option_selected_observer)
            .add_observer(populate_recipe_list_window);
    }
}
