#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneSystemSet {
    Start,
    Load,
    Ready,
}
