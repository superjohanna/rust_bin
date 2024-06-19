use bevy::{prelude::*, window::WindowResized};

use crate::{
    components::{
        board_marker::BoardMarker, coordinates::CoordinateU16, tile_cover_marker::TileCoverMarker,
    },
    helpers::adaptive_tile_size,
    options::{BoardOptions, BoardPosition, TileSize},
    resources::{board::Board, bounds::Bounds2},
};

/// Changes tile size and position if window size changed
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn rescale(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut board_component: Query<
        (&Children, &mut Transform),
        (With<BoardMarker>, Without<TileCoverMarker>),
    >,
    mut tiles: Query<
        (
            &mut Transform,
            &mut Sprite,
            &CoordinateU16,
            Option<&Children>,
        ),
        (Without<BoardMarker>, Without<TileCoverMarker>),
    >,
    mut covers: Query<(&mut Sprite, &mut Transform), (With<TileCoverMarker>, Without<BoardMarker>)>,
    mut board: Option<ResMut<Board>>,
    mut resize_event: EventReader<WindowResized>,
    windows: Query<Option<&mut Window>>,
    options: Option<Res<BoardOptions>>,
) {
    if resize_event.read().count() == 0 {
        return;
    }

    let tile_map = match &board {
        Some(t) => &t.tile_map,
        None => return,
    };

    #[cfg(not(feature = "debug"))]
    {
        let window_count = windows.iter().count();
        if window_count != 1 {
            bevy::log::error!("Can't have more or less than one window.");
            app_exit_events.send(bevy::app::AppExit);
        }
    }

    #[cfg(not(feature = "debug"))]
    let window = windows.single().unwrap();

    // I think the world view thingie counts as a window so we have to do this when debugging
    #[cfg(feature = "debug")]
    let window = windows.iter().next().unwrap().unwrap();

    let options = match options {
        Some(t) => t.clone(),
        None => Default::default(),
    };

    // Compute the tile size
    let tile_size = match options.tile_size {
        TileSize::Fixed(v) => v,
        TileSize::Adaptive { min, max } => {
            adaptive_tile_size(window, (min, max), (*tile_map.width(), *tile_map.height()))
        }
    };

    // Compute the size of all tiles added together
    let board_size = Vec2::new(
        *tile_map.width() as f32 * tile_size,
        *tile_map.height() as f32 * tile_size,
    );

    // Compute board position
    let board_position = match options.position {
        BoardPosition::Centered { offset } => {
            Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
        }
        BoardPosition::Custom(p) => p,
    };

    let (children_id, mut board_transform) = match board_component.iter_mut().next() {
        Some(t) => t,
        None => return,
    };

    *board_transform = Transform::from_translation(board_position);

    for child_id in children_id.iter() {
        let (mut transform, mut sprite_parent, coordinate, children) =
            match tiles.get_mut(*child_id) {
                Ok(t) => t,
                Err(_) => continue,
            };
        sprite_parent.custom_size = Some(Vec2::splat(tile_size - options.tile_padding));
        *transform = Transform::from_xyz(
            coordinate.x as f32 * tile_size + tile_size * 0.5,
            coordinate.y as f32 * tile_size + tile_size * 0.5,
            crate::BACKGROUND_Z,
        );

        if let Some(children) = children {
            for cover_id in children.iter() {
                let (mut sprite_cover, mut transform_cover) = match covers.get_mut(*cover_id) {
                    Ok(t) => t,
                    Err(_) => continue,
                };

                sprite_cover.custom_size = Some(Vec2::splat(tile_size - options.tile_padding));
                *transform_cover = Transform::from_xyz(0., 0., crate::FOREGROUND_Z)
            }
        }
    }

    // unwrap here is okay because we already return if this is none
    board.as_mut().unwrap().tile_size = tile_size;
    board.as_mut().unwrap().bounds = Bounds2 {
        origin: board_position.xy(),
        size: board_size,
    }
}
