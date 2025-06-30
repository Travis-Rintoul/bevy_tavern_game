use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Bundle)]
pub struct ListLayoutBundle {}

impl Default for ListLayoutBundle {
    fn default() -> Self {
        ListLayoutBundle {}
    }
}

impl ListLayoutBundle {
    pub fn spawn<T>(
        &self,
        commands: &mut RelatedSpawnerCommands<ChildOf>,
        item_records: &Vec<T>,
        item_content: impl Fn(&mut RelatedSpawnerCommands<ChildOf>, &T),
    ) -> Entity {
        commands
            .spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb_u8(0, 0, 255)),
            ))
            .with_children(|builder| {
                item_records.iter().for_each(|item| {
                    builder
                        .spawn(Node {
                            width: Val::Percent(20.0),
                            height: Val::Px(100.0),
                            ..Default::default()
                        })
                        .with_children(|spawner| item_content(spawner, item));
                });
            })
            .id()
    }
}

/*
    Example:
    ListLayoutBundle::default().spawn(parent, &items, |builder, item| {
        builder.spawn(Text::new(*item));
    });
*/
