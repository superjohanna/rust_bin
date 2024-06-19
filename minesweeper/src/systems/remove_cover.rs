use bevy::prelude::*;

use crate::components::{tile_cover_marker::TileCoverMarker, uncover_marker::UncoverMarker};

/// Removes the cover child of tiles that are marked with the [crate::components::uncover_marker::UncoverMarker] component
pub fn remove_cover(
    mut commands: Commands,
    tiles: Query<(Entity, &Children), With<UncoverMarker>>,
    covers: Query<Entity, With<TileCoverMarker>>,
) {
    if tiles.is_empty() {
        return;
    }

    for (parent, children) in tiles.iter() {
        for child in children.iter() {
            let child = match covers.get(*child) {
                Ok(t) => t,
                Err(_) => continue,
            };
            commands.entity(parent).remove_children(&[child]);
            commands.entity(child).despawn_recursive();
        }
    }

    tiles.iter().for_each(|(parent_entity, _)| {
        commands.entity(parent_entity).remove::<UncoverMarker>();
    });
}
