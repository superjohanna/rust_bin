use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, States)]
pub enum MinesweeperState {
    #[default]
    NewGame,
    Running,
}
