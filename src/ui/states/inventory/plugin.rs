use bevy::{prelude::*, state::state_scoped::clear_state_scoped_entities};

use crate::{
    core::{
        InterfaceFlowSet, InterfaceSetup, InventoryWindow, Owner, Player,
        PlayerOpenedInventoryUIEvent, RequestInventoryUIPopulationEvent, UIState,
        clear_interface_setup, set_interface_setup,
    },
    ui::InventoryWindowBundle,
};

pub struct InventroyUIPlugin;

impl Plugin for InventroyUIPlugin {
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
            emit_inventory_window_population_request.in_set(InterfaceFlowSet::AfterHook),
        )
        .add_systems(
            OnExit(UIState::Inventory),
            (
                clear_state_scoped_entities::<UIState>,
                clear_interface_setup,
            ),
        );
    }
}

fn condtion(ui_state: Res<State<UIState>>, interface_setup: Res<InterfaceSetup>) -> bool {
    *ui_state.get() == UIState::Inventory && !interface_setup.0
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Text::from("Inventory"),
        StateScoped(UIState::Inventory),
        InventoryWindowBundle::new(Owner::Player),
    ));
}

fn emit_inventory_window_population_request(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut events: EventReader<PlayerOpenedInventoryUIEvent>,
    inventory_window_query: Query<(Entity, Option<&Owner>), With<InventoryWindow>>,
) {
    for _ in events.read() {
        let Ok(player_entity) = player_query.single() else {
            // TODO: make message a constant
            panic!("Unable to find player");
        };

        for (window_entity, owner) in &inventory_window_query {
            if matches!(owner, Some(Owner::Player)) {
                // Tell the UI to display the ui items
                commands
                    .entity(window_entity)
                    .trigger(RequestInventoryUIPopulationEvent {
                        window_entity: window_entity,
                        inventory_entity: player_entity,
                    });
            }
        }
    }
}
