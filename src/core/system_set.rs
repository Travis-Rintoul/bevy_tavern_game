use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneSystemSet {
    Start,
    Load,
    Ready,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceFlowSet {
    InputHook,
    InputBufferHook,
    EntryHook,
    BeforeHook,
    ActionHook,
    AfterHook,
    PostAfterHook,
}
