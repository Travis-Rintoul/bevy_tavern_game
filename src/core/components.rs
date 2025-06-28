use bevy::{ecs::component::Component, time::Timer};

use crate::core::RecipeID;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct StoveDevice;

#[derive(Component, Default)]
pub struct StoveSlot;

#[derive(Component, Default)]
pub struct Device;

#[derive(Component)]
pub struct Cooking {
    pub timer: Timer,
    pub recipe_id: RecipeID,
}
