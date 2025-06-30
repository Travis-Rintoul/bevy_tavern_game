use bevy::prelude::*;

use crate::core::{ItemID, ItemStack, RecipeID};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Device;

#[derive(Component, Default)]
pub struct StoveDevice;

#[derive(Component, Default)]
pub struct StoveSlot;

#[derive(Component, Default)]
pub enum Owner {
    #[default]
    Other,
    Player,
    Npc(Entity),
}

#[derive(Component)]
pub struct Cooking {
    pub timer: Timer,
    pub recipe_id: RecipeID,
}

#[derive(Component, Debug)]
pub struct InventoryItem;

#[derive(Component, Debug)]
pub struct InventoryWindow;

#[derive(Component, Debug, Default)]
pub struct InventoryItemWindow {
    pub item_id: ItemID,
    pub item_count: u32,
}

#[derive(Component, Debug, Default)]
pub struct Inventory {
    stacks: Vec<ItemStack>,
    max_inventory_stack: u32,
}

impl Inventory {
    pub fn add_item(&mut self, item: ItemID, count: u32) {
        let existing_stack: Option<&mut ItemStack> =
            self.stacks.iter_mut().find(|stack| stack.item_id == item);

        if let Some(stack) = existing_stack {
            stack.item_count += count;
        } else {
            self.stacks.push(ItemStack::new(item, count));
        }
    }

    pub fn contains_item(self, item: ItemID) -> bool {
        self.stacks.iter().any(|stack| stack.item_id == item)
    }
}

impl<'a> IntoIterator for &'a Inventory {
    type Item = &'a ItemStack;
    type IntoIter = std::slice::Iter<'a, ItemStack>;

    fn into_iter(self) -> Self::IntoIter {
        self.stacks.iter()
    }
}

impl<'a> IntoIterator for &'a mut Inventory {
    type Item = &'a mut ItemStack;
    type IntoIter = std::slice::IterMut<'a, ItemStack>;

    fn into_iter(self) -> Self::IntoIter {
        self.stacks.iter_mut()
    }
}

#[derive(Component, Debug, Default)]
pub struct CraftingOptions {
    recipes: Vec<RecipeID>,
}
