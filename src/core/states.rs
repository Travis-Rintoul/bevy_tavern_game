use bevy::state::state::States;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum Scenes {
    #[default]
    TestScene,
}

#[allow(dead_code)]
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum UIState {
    #[default]
    Hidden,
    Exploration,
    Dialog,
    Inventory,
    DeviceStove,
}
