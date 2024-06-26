use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum PauseState {
    #[default]
    NotPaused,
    Paused,
}
