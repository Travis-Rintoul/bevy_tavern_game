use bevy::prelude::*;

use crate::core::{
    ActiveDeviceResource, CraftingFinishedEvent, InterfaceFlowSet, InterfaceSetup,
    PlayerClosedDeviceUIEvent, PlayerClosedInventoryUIEvent, PlayerMovedEvent,
    PlayerOpenedDeviceUIEvent, PlayerOpenedInventoryUIEvent, RequestInventoryUIPopulationEvent,
    RequestRecipeUIPopulationEvent, Scenes, StartCraftingRequestEvent, UIState,
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
            .add_event::<PlayerOpenedDeviceUIEvent>()
            .add_event::<PlayerClosedDeviceUIEvent>()
            .add_event::<PlayerOpenedInventoryUIEvent>()
            .add_event::<PlayerClosedInventoryUIEvent>()
            .add_event::<RequestInventoryUIPopulationEvent>()
            .add_event::<RequestRecipeUIPopulationEvent>()
            .add_event::<StartCraftingRequestEvent>()
            .add_event::<CraftingFinishedEvent>();
    }
}
