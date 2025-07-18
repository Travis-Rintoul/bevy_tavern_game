use bevy::prelude::*;

use crate::core::{
    CraftingProgress, CraftingTask, DeviceType, InteractableType, ItemID, ItemStack, NPCType,
    RecipeID,
};

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
    Device(Entity),
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

#[derive(Component, Debug)]
pub struct RecipeListWindow;

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

// TODO: flesh out inventory logic
impl Inventory {
    pub fn add_item(&mut self, item: ItemID, count: u32) {
        self.stacks.push(ItemStack::new(item, count));
    }

    pub fn contains_item(&self, item: ItemID) -> bool {
        self.stacks.iter().any(|stack| stack.item_id == item)
    }

    pub fn remove_item(&mut self, item: ItemID, count: u32) {
        let mut stack_search = self
            .stacks
            .iter_mut()
            .find(|predicate| predicate.item_id == item);

        match stack_search {
            None => warn!(
                "Inventory item not found: Unable to locate item with ID {:?}",
                item
            ),
            Some(stack) => stack.item_count -= count,
        }
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

#[derive(Component, Debug)]
pub struct RecipeList;

#[derive(Component, Debug)]
pub struct RecipeWindow;

#[derive(Component, Debug)]
pub struct RecipeListOption(pub RecipeID);

#[derive(Component, Default)]
pub struct CraftingStation;

#[derive(Component, Default)]
pub struct Station;

#[derive(Component)]
pub struct CraftButton(pub RecipeID);

#[derive(Component)]
pub struct CraftingQueue {
    pub timer: Timer,
    pub paused: bool,
    pub queue: Vec<CraftingTask>,
    pub current_progress: f32,
}

impl Default for CraftingQueue {
    fn default() -> Self {
        CraftingQueue {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            paused: true,
            queue: Vec::default(),
            current_progress: f32::default(),
        }
    }
}

#[derive(Component)]
pub struct Customer;

#[derive(Component, Default)]
pub struct CustomerOrder(pub ItemID);

#[derive(Component)]
pub struct Interactable {
    pub interactable_type: InteractableType,
}

impl Interactable {
    pub fn npc(npc_type: NPCType) -> Self {
        Interactable {
            interactable_type: InteractableType::NPC(npc_type),
        }
    }

    pub fn device(device_type: DeviceType) -> Self {
        Interactable {
            interactable_type: InteractableType::Device(device_type),
        }
    }
}

#[derive(Component, Default)]
pub struct CraftingStatusWindow {
    owner: Owner,
}

impl CraftingStatusWindow {
    pub fn new(owner: Owner) {
        CraftingStatusWindow { owner: owner };
    }
}
