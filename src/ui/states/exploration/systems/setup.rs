use bevy::prelude::*;

use crate::core::UIState;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..Default::default()
        },
        Text::from("Exploration"),
        StateScoped(UIState::Exploration),
    ));
}
