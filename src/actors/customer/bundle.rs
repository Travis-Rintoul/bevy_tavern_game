use bevy::prelude::*;

use crate::core::{Customer, CustomerOrder, Interactable, Inventory, NPCType};

#[derive(Bundle)]
pub struct CustomerActorBundle {
    marker: Customer,
    inventory: Inventory,
    order: CustomerOrder,
    interactable: Interactable,
}

impl Default for CustomerActorBundle {
    fn default() -> Self {
        CustomerActorBundle {
            marker: Customer,
            inventory: Inventory::default(),
            order: CustomerOrder::default(),
            interactable: Interactable::npc(NPCType::Customer),
        }
    }
}
