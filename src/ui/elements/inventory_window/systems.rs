use bevy::{color::palettes::css::GOLD, prelude::*};

use crate::{
    core::{Inventory, InventoryWindow, InventoryWindowPopulationRequestEvent},
    ui::InventoryItemWindowBundle,
};

pub fn populate_inventory_window(
    mut commands: Commands,
    children_query: Query<&Children>,
    inventory_query: Query<&Inventory>,
    inventory_window_query: Query<&InventoryWindow>,
    mut events: EventReader<InventoryWindowPopulationRequestEvent>,
) {
    for event in events.read() {
        let Ok(inventory) = inventory_query.get(event.inventory_entity) else {
            continue;
        };

        let Ok(_) = inventory_window_query.get(event.window_entity) else {
            continue;
        };

        if let Ok(children) = children_query.get(event.window_entity) {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        commands
            .entity(event.window_entity)
            .with_children(|parent| {
                inventory.into_iter().for_each(|stack| {
                    parent
                        .spawn(InventoryItemWindowBundle::from_stack(stack))
                        .with_children(|item_window| {
                            // Spawn stack count
                            item_window.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(0.0),
                                    right: Val::Px(0.0),
                                    ..Default::default()
                                },
                                Text::new(format!("{}", stack.item_count)),
                                TextColor(GOLD.into()),
                            ));
                        });
                });
            });
    }
}
