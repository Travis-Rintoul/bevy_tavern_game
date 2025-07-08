use bevy::prelude::*;

use crate::core::UIState;

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Relative,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            Text::from("Exploration"),
            StateScoped(UIState::Exploration),
        ))
        .with_children(|parent| {
            parent.spawn((Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Px(200.0),
                height: Val::Auto,
                ..Default::default()
            },));
        });
}
