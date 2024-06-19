use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::{
    events::{TileFlagEvent, TileUncoverEvent},
    resources::board::Board,
};

pub fn input(
    windows: Query<&Window>,
    board: Res<Board>,
    mut button_event_read: EventReader<MouseButtonInput>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut tile_trigger_ewr: EventWriter<TileUncoverEvent>,
    mut tile_flag_ewr: EventWriter<TileFlagEvent>,
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
    let window = windows.single();

    #[cfg(feature = "debug")]
    let window = if let Ok(t) = windows.get_single() {
        t
    } else {
        return;
    };

    for event in button_event_read.read() {
        if let ButtonState::Pressed = event.state {
            if let Some(click_position) = window.cursor_position() {
                if let Some(tile_coordinates) = board.cursor_position(window, click_position) {
                    match event.button {
                        MouseButton::Left => {
                            bevy::log::debug!("Trying to uncover tile on {}", tile_coordinates);
                            tile_trigger_ewr.send(TileUncoverEvent(tile_coordinates));
                        }
                        MouseButton::Right => {
                            bevy::log::debug!("Trying to flag tile on {}", tile_coordinates);
                            tile_flag_ewr.send(TileFlagEvent(tile_coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
