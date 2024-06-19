use std::collections::HashMap;

use crate::{
    components::{
        board_marker::BoardMarker, coordinates::CoordinateU16, tile_cover_marker::TileCoverMarker,
        tile_marker::TileMarker,
    },
    helpers::adaptive_tile_size,
    options::{self, BoardOptions, BoardPosition, TileSize},
    resources::{
        board::Board,
        bounds::Bounds2,
        plugin_options::{self, PluginOptions},
        tile::Tile,
        tile_map::TileMap,
    },
    states::plugin_state::MinesweeperState,
    texture_handles::TextureHandles,
    BACKGROUND_Z, FOREGROUND_Z,
};

use super::MinesweeperPlugin;
use bevy::prelude::*;

impl<
        TyRunState: States + PartialEq,
        TyPauseState: States + PartialEq,
        TyResetEvent: Event + Clone,
        TyWonEvent: Event + Clone,
        TyLostEvent: Event + Clone,
    > MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
{
    pub fn create_board(
        mut commands: Commands,
        windows: Query<Option<&mut Window>>,
        options: Option<Res<BoardOptions>>,
        textures: Option<Res<TextureHandles>>,
        old_board: Query<Entity, With<BoardMarker>>,
        mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
        mut next_state: ResMut<NextState<MinesweeperState>>,
    ) {
        #[cfg(not(feature = "debug"))]
        {
            let window_count = windows.iter().count();
            if window_count != 1 {
                bevy::log::error!("Can't have more or less than one window.");
                app_exit_events.send(bevy::app::AppExit);
                return;
            }
        }

        #[cfg(not(feature = "debug"))]
        let window = windows.single().unwrap();

        #[cfg(feature = "debug")]
        let window = windows.iter().next().unwrap().unwrap();

        // Despawn old board if it exists
        // Was previously in restart but caused flickering
        if let Ok(t) = old_board.get_single() {
            commands.entity(t).despawn_recursive();
        }

        if textures.is_none() {
            bevy::log::error!("No texture handles. Can't continue.");
            app_exit_events.send(bevy::app::AppExit);
            return;
        }

        let textures = textures.unwrap();

        let options = match options {
            Some(t) => t.clone(),
            None => Default::default(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.spread_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        bevy::log::info!("{}", tile_map);

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => {
                adaptive_tile_size(window, (min, max), (*tile_map.width(), *tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            *tile_map.width() as f32 * tile_size,
            *tile_map.height() as f32 * tile_size,
        );

        bevy::log::info!("board_size: {}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            options::BoardPosition::Custom(p) => p,
        };

        let mut covered = HashMap::new();
        let mut flagged = HashMap::new();

        commands
            .spawn((
                Name::new("Board"),
                SpatialBundle {
                    transform: Transform::from_translation(board_position),
                    ..Default::default()
                },
                BoardMarker,
            ))
            .with_children(|parent| {
                let mut count = 0;
                for (index, tile) in tile_map.iter().enumerate() {
                    let coordinate = CoordinateU16 {
                        x: (index % *tile_map.width() as usize) as u16,
                        y: (index / *tile_map.width() as usize) as u16,
                    };
                    let mut covered_id = None;
                    let mut entity_commands = parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::splat(tile_size - options.tile_padding)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                (index % *tile_map.width() as usize) as f32 * tile_size
                                    + tile_size * 0.5,
                                (index / *tile_map.width() as usize) as f32 * tile_size
                                    + tile_size * 0.5,
                                BACKGROUND_Z,
                            ),
                            texture: match tile {
                                Tile::Bomb => textures.tile_bomb.clone(),
                                Tile::Empty => match tile_map.bomb_count_at_index(index) {
                                    0 => textures.tile_0.clone(),
                                    1 => textures.tile_1.clone(),
                                    2 => textures.tile_2.clone(),
                                    3 => textures.tile_3.clone(),
                                    4 => textures.tile_4.clone(),
                                    5 => textures.tile_5.clone(),
                                    6 => textures.tile_6.clone(),
                                    7 => textures.tile_7.clone(),
                                    8 => textures.tile_8.clone(),
                                    _ => panic!("Invalid number of bomb neighbours"),
                                },
                            },
                            ..Default::default()
                        },
                        Name::new(format!("Tile: {}", index)),
                        coordinate,
                        TileMarker,
                    ));
                    entity_commands.with_children(|parent| {
                        covered_id = Some(
                            parent
                                .spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::WHITE,
                                            custom_size: Some(Vec2::splat(
                                                tile_size - options.tile_padding,
                                            )),
                                            ..Default::default()
                                        },
                                        texture: textures.tile_base.clone(),
                                        transform: Transform::from_xyz(0., 0., FOREGROUND_Z),
                                        ..Default::default()
                                    },
                                    Name::new(format!("Tile Cover: {}", index)),
                                    coordinate,
                                    TileCoverMarker,
                                ))
                                .id(),
                        );
                    });
                    covered.insert(coordinate, entity_commands.id());
                    flagged.insert(
                        coordinate,
                        (entity_commands.id(), covered_id.unwrap(), false),
                    );
                    count += 1;
                }
                bevy::log::debug!("Ran loop {} times", count);
            });
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                origin: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered,
            flagged,
            flag_count: 0,
        });
        next_state.set(MinesweeperState::Running);
    }

    pub fn reset_event_handler(
        mut state: ResMut<NextState<MinesweeperState>>,
        mut events: EventReader<TyResetEvent>,
    ) {
        if events.read().count() == 0 {
            return;
        }
        state.set(MinesweeperState::NewGame);
    }
}
