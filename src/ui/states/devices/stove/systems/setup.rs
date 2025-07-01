use bevy::prelude::*;

use crate::{
    core::{ActiveDeviceResource, Owner, UIState},
    ui::RecipeListWindowBundle,
};

pub fn setup(mut commands: Commands, active_device: Res<ActiveDeviceResource>) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            StateScoped(UIState::DeviceStove),
            BackgroundColor(Color::srgb_u8(255, 0, 0)),
        ))
        .with_children(|parent| {
            // Side Bar
            parent
                .spawn(Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                })
                .with_children(|parent| {
                    if let Some(entity) = active_device.0 {
                        parent.spawn((
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            RecipeListWindowBundle::default(),
                            Owner::Device(entity),
                        ));
                    } else {
                        parent.spawn(RecipeListWindowBundle::default());
                    }
                });

            // Main bar
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                Text::from("Stove Device"),
            ));
        });
}
