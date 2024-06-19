use bevy::prelude::*;

/// Determines wether the tiles should be fixed (don't change with window scaling) or adaptive (change with window resizing but only above a minimum and below a maximum)
#[derive(Debug, Clone, Resource)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

/// The position of the board
#[derive(Debug, Clone, Resource)]
pub enum BoardPosition {
    /// Center the board with an offset
    Centered { offset: Vec3 },
    /// Fully custom position
    Custom(Vec3),
}

/// Vairous options relating to the board
#[derive(Debug, Clone, Resource)]
pub struct BoardOptions {
    /// Board size
    pub map_size: (u16, u16),
    /// Bomb count
    pub bomb_count: u16,
    /// Board world position
    pub position: BoardPosition,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
    /// Does the board generate a safe place to start
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Vec3::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
