use bevy::prelude::*;

use crate::core::{Customer, CustomerOrder, Inventory};

#[derive(Bundle)]
pub struct CustomActorBundle {
    marker: Customer,
    inventory: Inventory,
    order: CustomerOrder,
}

impl Default for CustomActorBundle {
    fn default() -> Self {
        CustomActorBundle {
            marker: Customer,
            inventory: Inventory::default(),
            order: CustomerOrder::default(),
        }
    }
}
