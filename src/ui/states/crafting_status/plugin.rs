use bevy::{
    prelude::*, state::state_scoped::clear_state_scoped_entities, text::cosmic_text::Change,
};

use crate::core::{
    CraftingQueue, CraftingStatusWindow, InterfaceFlowSet, InterfaceSetup, ItemID, Owner, Player,
    PlayerOpenedCraftingStatusUIEvent, PlayerOpenedInventoryUIEvent,
    RequestCraftingStatusUIPopulationEvent, UIState, clear_interface_setup, set_interface_setup,
};

pub struct CraftingStatusUIPlugin;

impl Plugin for CraftingStatusUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_interface_setup, setup)
                .chain()
                .run_if(condtion)
                .in_set(InterfaceFlowSet::ActionHook),
        )
        .add_systems(
            Update,
            on_crafting_status_population_event.in_set(InterfaceFlowSet::AfterHook),
        )
        .add_systems(
            OnExit(UIState::CraftingStatus),
            (
                clear_state_scoped_entities::<UIState>,
                clear_interface_setup,
            ),
        );
    }
}

fn condtion(ui_state: Res<State<UIState>>, interface_setup: Res<InterfaceSetup>) -> bool {
    *ui_state.get() == UIState::CraftingStatus && !interface_setup.0
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Text::from("Crafting Status"),
        StateScoped(UIState::CraftingStatus),
        CraftingStatusWindow::default(),
    ));
}

fn on_crafting_status_population_event(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut events: EventReader<PlayerOpenedCraftingStatusUIEvent>,
    crafting_window_query: Query<(Entity, Option<&Owner>), With<CraftingStatusWindow>>,
) {
    for _ in events.read() {
        let Ok(player_entity) = player_query.single() else {
            warn!("Unable to find player");
            return;
        };

        for (window_entity, owner) in &crafting_window_query {
            commands
                .entity(window_entity)
                .trigger(RequestCraftingStatusUIPopulationEvent);
        }
    }
}

fn update_ui(
    window_query: Query<Entity, With<CraftingStatusWindow>>,
    query: Query<&CraftingQueue, (With<CraftingQueue>, Changed<CraftingQueue>)>,
) {
    let test = Vec::<(ItemID, f32)>::default();

    let queues: Vec<(ItemID, f32, bool)> = query
        .iter()
        .filter_map(|predicate| {
            predicate.queue.first().map(|first_task| {
                (
                    first_task.result_item_id,
                    predicate.current_progress,
                    predicate.paused,
                )
            })
        })
        .collect::<ItemID, f32, bool>();
}
