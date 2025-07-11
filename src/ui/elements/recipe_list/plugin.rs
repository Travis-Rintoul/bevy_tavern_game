use bevy::prelude::*;

use crate::ui::elements::recipe_list::systems::{
    on_populate_recipe_list_request, option_selected_observer,
};

pub struct RecipeListWindowUIElementPlugin;

impl Plugin for RecipeListWindowUIElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_populate_recipe_list_request);
    }
}
