use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Bundle)]
pub struct LayoutWrapperBundle {
    node: Node,
}

impl Default for LayoutWrapperBundle {
    fn default() -> Self {
        LayoutWrapperBundle {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
        }
    }
}

impl LayoutWrapperBundle {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        content: impl FnOnce(&mut RelatedSpawnerCommands<ChildOf>),
    ) -> Entity {
        commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            })
            .with_children(|builder| {
                builder
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    })
                    .with_children(content);
            })
            .id()
    }
}
