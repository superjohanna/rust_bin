use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum Tile {
    Bomb,
    Empty,
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }
}
