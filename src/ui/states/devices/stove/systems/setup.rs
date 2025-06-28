use bevy::prelude::*;

use crate::core::UIState;

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            StateScoped(UIState::DeviceStove),
            BackgroundColor(Color::srgb_u8(255, 0, 0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                Text::from("Stove Device"),
            ));
        });
}
