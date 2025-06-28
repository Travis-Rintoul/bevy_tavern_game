use bevy::prelude::*;

use crate::core::{
    PlayerClosedDeviceInterfaceEvent, PlayerMovedEvent, PlayerOpenedDeviceInterfaceEvent, Scenes,
    UIState,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Init States
        app.init_state::<Scenes>().init_state::<UIState>();

        // Init Events
        app.add_event::<PlayerMovedEvent>()
            .add_event::<PlayerOpenedDeviceInterfaceEvent>()
            .add_event::<PlayerClosedDeviceInterfaceEvent>();
    }
}
