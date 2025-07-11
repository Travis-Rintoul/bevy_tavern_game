use bevy::prelude::*;

use crate::core::{
    ActiveInteractable, Interactable, InteractableType, Player, PlayerInteractButtonPressedEvent,
    PlayerMovedEvent, PlayerOpenedDeviceUIEvent, PlayerOpenedDialogEvent,
};

pub fn on_player_interaction_event(
    interactable_resource: Option<Res<ActiveInteractable>>,
    mut events: EventReader<PlayerInteractButtonPressedEvent>,
    mut device_opened_event: EventWriter<PlayerOpenedDeviceUIEvent>,
    mut dialog_opened_event: EventWriter<PlayerOpenedDialogEvent>,
) {
    let Some(interactable) = interactable_resource else {
        println!("Unable to find active interactable in proximity");
        return;
    };

    for event in events.read() {
        match &interactable.interactable_type {
            InteractableType::Device(device_type) => {
                device_opened_event.write(PlayerOpenedDeviceUIEvent {
                    device: interactable.entity,
                    device_type: device_type.clone(),
                    needs_recipes: true,
                });
            }
            InteractableType::NPC(npc_type) => {
                dialog_opened_event.write(PlayerOpenedDialogEvent);
            }
        }
    }
}

pub fn on_player_proximity_change(
    mut commands: Commands,
    mut events: EventReader<PlayerMovedEvent>,
    player_query: Query<&Transform, With<Player>>,
    interactable_query: Query<(&Transform, Entity, &Interactable)>,
) {
    for _ in events.read() {
        let Some(player_transform) = player_query.single().ok() else {
            return;
        };

        let closest = interactable_query
            .iter()
            .map(|predicate| {
                (
                    predicate
                        .0
                        .translation
                        .distance(player_transform.translation),
                    predicate.1,
                    predicate.2,
                )
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        if let Some((_, entity, meta)) = closest {
            commands.insert_resource(ActiveInteractable {
                entity: entity,
                interactable_type: meta.interactable_type.clone(),
            });
        }
    }
}
