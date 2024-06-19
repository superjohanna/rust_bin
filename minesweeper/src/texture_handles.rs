use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct TextureHandles {
    pub tile_0: Handle<Image>,
    pub tile_1: Handle<Image>,
    pub tile_2: Handle<Image>,
    pub tile_3: Handle<Image>,
    pub tile_4: Handle<Image>,
    pub tile_5: Handle<Image>,
    pub tile_6: Handle<Image>,
    pub tile_7: Handle<Image>,
    pub tile_8: Handle<Image>,
    pub tile_base: Handle<Image>,
    pub tile_bomb: Handle<Image>,
    pub tile_flag: Handle<Image>,
}
