use bevy::prelude::*;

use crate::{
    components::tile_cover_marker::TileCoverMarker, events::TileFlagEvent, resources::board::Board,
    texture_handles::TextureHandles,
};

pub fn tile_flag_event_handler(
    mut board: ResMut<Board>,
    mut tile_flag_evr: EventReader<TileFlagEvent>,
    mut query: Query<&mut Handle<Image>, With<TileCoverMarker>>,
    textures: Res<TextureHandles>,
) {
    for event in tile_flag_evr.read() {
        if let Some((_, cover, bool)) = board.try_toggle_flag(event) {
            let mut texture = match query.get_mut(cover) {
                Ok(t) => t,
                Err(_) => continue,
            };

            match bool {
                true => {
                    *texture = textures.tile_flag.clone();
                    board.flag_count += 1;
                }

                false => {
                    *texture = textures.tile_base.clone();
                    board.flag_count -= 1;
                }
            };
        }
    }
}
