use bevy::prelude::*;

use crate::core::{
    InventoryWindowPopulationRequestEvent, PlayerClosedDeviceInterfaceEvent,
    PlayerClosedInventoryScreenEvent, PlayerMovedEvent, PlayerOpenInventoryScreenEvent,
    PlayerOpenedDeviceInterfaceEvent, Scenes, UIState,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Init States
        app.init_state::<Scenes>().init_state::<UIState>();

        // Init Events
        app.add_event::<PlayerMovedEvent>()
            .add_event::<PlayerOpenedDeviceInterfaceEvent>()
            .add_event::<PlayerClosedDeviceInterfaceEvent>()
            .add_event::<PlayerOpenInventoryScreenEvent>()
            .add_event::<PlayerClosedInventoryScreenEvent>()
            .add_event::<InventoryWindowPopulationRequestEvent>();
    }
}
