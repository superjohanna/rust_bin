use bevy::prelude::*;

use crate::components::coordinates::CoordinateU16;

/// Event that is sent when a tile is left clicked. Contains the board coordinates of the tile
#[derive(Debug, Clone, Copy, Event)]
pub(crate) struct TileUncoverEvent(pub CoordinateU16);

impl std::ops::Deref for TileUncoverEvent {
    type Target = CoordinateU16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Event that is sent when a tile is right clicked. Contains the board coordinates of the tile
#[derive(Debug, Clone, Copy, Event)]
pub(crate) struct TileFlagEvent(pub CoordinateU16);

impl std::ops::Deref for TileFlagEvent {
    type Target = CoordinateU16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
