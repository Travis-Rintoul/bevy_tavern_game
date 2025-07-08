use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Bundle)]
pub struct SidebarLayoutBundle {}

impl Default for SidebarLayoutBundle {
    fn default() -> Self {
        SidebarLayoutBundle {}
    }
}

impl SidebarLayoutBundle {
    pub fn spawn(
        &self,
        commands: &mut RelatedSpawnerCommands<ChildOf>,
        sidebar_content: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
        main_content: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
    ) -> Entity {
        commands
            .spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb_u8(0, 0, 255)),
            ))
            .with_children(|builder| {
                builder
                    .spawn(Node {
                        width: Val::Percent(20.0),
                        height: Val::Px(100.0),
                        ..Default::default()
                    })
                    .with_children(sidebar_content);

                builder
                    .spawn(Node {
                        width: Val::Percent(80.0),
                        height: Val::Px(100.0),
                        ..Default::default()
                    })
                    .with_children(main_content);
            })
            .id()
    }
}
