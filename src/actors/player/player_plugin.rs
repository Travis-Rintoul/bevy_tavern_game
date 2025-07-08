use bevy::prelude::*;

use crate::{
    actors::player::systems::{
        close_menus, player_handle_ui_input, player_movement, stove_interaction,
    },
    core::{
        Customer, CustomerOrder, InterfaceFlowSet, Inventory, PLAYER_PROXIMITY_DIALOG_CHECK,
        Player, player_exists,
    },
};

pub struct ActorPlayerPlugin;

impl Plugin for ActorPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                stove_interaction,
                player_handle_ui_input,
                handle_proximity_dialog,
            )
                .in_set(InterfaceFlowSet::InputHook),
        );

        app.add_systems(Update, (player_movement, close_menus).run_if(player_exists));
    }
}

fn handle_proximity_dialog(
    button: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&Transform, &Inventory), (With<Player>, Without<Customer>)>,
    customer_query: Query<(&Transform, &CustomerOrder), (With<Customer>, Without<Player>)>,
) {
    for (customer_transform, order) in customer_query {
        let distance = customer_transform
            .translation
            .distance(player_query.0.translation);

        if button.just_pressed(KeyCode::KeyE) && distance < PLAYER_PROXIMITY_DIALOG_CHECK {
            if player_query.1.contains_item(order.0) {
                println!("Thanks for my order :)");
            } else {
                println!("You don't have my order");
            }
        }
    }
}
