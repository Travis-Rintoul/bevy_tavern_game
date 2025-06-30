use bevy::prelude::*;

use crate::{
    core::{Owner, UIState},
    ui::InventoryWindowBundle,
};

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            Text::from("Inventory"),
            BackgroundColor(Color::srgb_u8(255, 0, 0)),
            StateScoped(UIState::Inventory),
        ))
        .with_children(|parent| {
            parent.spawn(InventoryWindowBundle::new(Owner::Player));
        });
}
