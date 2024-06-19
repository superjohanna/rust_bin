use std::collections::HashMap;

use crate::components::coordinates::CoordinateU16;

use super::{bounds::Bounds2, tile_map::TileMap};
use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered: HashMap<CoordinateU16, Entity>,
    pub flagged: HashMap<CoordinateU16, (Entity, Entity, bool)>,
    pub flag_count: u16,
}

impl Board {
    pub fn cursor_position(&self, window: &Window, cursor_position: Vec2) -> Option<CoordinateU16> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = cursor_position - window_size / 2.;

        if !self.bounds.in_bounds(position) {
            return None;
        }

        // World space to board space
        let coordinates = position - self.bounds.origin;
        Some(CoordinateU16 {
            x: (coordinates.x / self.tile_size) as u16,
            // The position 1:1 is top left but the world origin is bottom left, so we have to flip position logic upside down
            y: self.tile_map.height() - (coordinates.y / self.tile_size) as u16 - 1,
        })
    }

    pub fn try_uncover(&mut self, coord: &CoordinateU16) -> Option<Entity> {
        self.covered.remove(coord)
    }

    pub fn try_toggle_flag(&mut self, coord: &CoordinateU16) -> Option<(Entity, Entity, bool)> {
        match self.flagged.get_mut(coord) {
            Some((tile, cover, bool)) => {
                *bool = !*bool;
                Some((*tile, *cover, *bool))
            }
            None => None,
        }
    }
}
