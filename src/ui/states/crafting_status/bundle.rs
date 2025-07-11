use bevy::prelude::*;

use crate::core::CraftingStatusWindow;

#[derive(Bundle, Default)]
pub struct CraftingStatusUIBundle {
    marker: CraftingStatusWindow,
}
