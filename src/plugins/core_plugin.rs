use bevy::prelude::*;

use crate::core::{
    ActiveDeviceResource, CraftingStationFinishedCraftingRequest,
    CraftingStationStartCraftingRequest, InterfaceFlowSet, InterfaceSetup,
    InventoryWindowPopulationRequestEvent, PlayerClosedDeviceInterfaceEvent,
    PlayerClosedInventoryScreenEvent, PlayerMovedEvent, PlayerOpenInventoryScreenEvent,
    PlayerOpenedDeviceInterfaceEvent, RecipeWindowPopulationRequestEvent, Scenes, UIState,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Init States
        app.init_state::<Scenes>().init_state::<UIState>();

        // Init Resources
        app.init_resource::<ActiveDeviceResource>()
            .init_resource::<InterfaceSetup>();

        // Configure System sets
        app.configure_sets(
            PreUpdate,
            (
                InterfaceFlowSet::InputHook,
                InterfaceFlowSet::EntryHook.after(InterfaceFlowSet::InputHook),
            ),
        );

        app.configure_sets(
            Update,
            (
                InterfaceFlowSet::BeforeHook.after(InterfaceFlowSet::EntryHook),
                InterfaceFlowSet::ActionHook.after(InterfaceFlowSet::BeforeHook),
                InterfaceFlowSet::AfterHook.after(InterfaceFlowSet::ActionHook),
            ),
        );

        app.configure_sets(
            PostUpdate,
            (InterfaceFlowSet::PostAfterHook.after(InterfaceFlowSet::AfterHook),),
        );

        // Init Events
        app.add_event::<PlayerMovedEvent>()
            .add_event::<PlayerOpenedDeviceInterfaceEvent>()
            .add_event::<PlayerClosedDeviceInterfaceEvent>()
            .add_event::<PlayerOpenInventoryScreenEvent>()
            .add_event::<PlayerClosedInventoryScreenEvent>()
            .add_event::<InventoryWindowPopulationRequestEvent>()
            .add_event::<RecipeWindowPopulationRequestEvent>()
            .add_event::<CraftingStationStartCraftingRequest>()
            .add_event::<CraftingStationFinishedCraftingRequest>();
    }
}
