use bevy::prelude::*;

use crate::core::{Customer, CustomerOrder, ItemID};

pub struct CustomerActorPlugin;

impl Plugin for CustomerActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_customer_order);
    }
}

fn set_customer_order(
    mut query: Query<&mut CustomerOrder, (With<Customer>, Added<CustomerOrder>)>,
) {
    for mut order in &mut query {
        order.0 = ItemID::CookedMeat;
    }
}
