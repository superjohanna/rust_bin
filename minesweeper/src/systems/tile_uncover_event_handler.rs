use std::ops::Index;

use bevy::{ecs::event::ManualEventReader, prelude::*};

use crate::{
    components::uncover_marker::UncoverMarker,
    events::TileUncoverEvent,
    resources::{board::Board, plugin_options::PluginOptions},
    MinesweeperPlugin,
};

impl<
        TyRunState: States + PartialEq,
        TyPauseState: States + PartialEq,
        TyResetEvent: Event + Clone,
        TyWonEvent: Event + Clone,
        TyLostEvent: Event + Clone,
    > MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
{
    /// Handles the uncovering of tiles and sends a lost event if it's a bomb
    pub(crate) fn tile_uncover_event_handler(
        plugin_options: Res<
            PluginOptions<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>,
        >,
        mut commands: Commands,
        mut board: ResMut<Board>,
        mut local_event_reader: Local<ManualEventReader<TileUncoverEvent>>,
        mut tile_trigger_events: ResMut<Events<TileUncoverEvent>>,
        mut lost_event_writer: EventWriter<TyLostEvent>,
    ) {
        let mut events = Vec::new();

        for event in local_event_reader.read(&tile_trigger_events) {
            // Returns true if entity is some
            if !add_uncover_marker(&mut commands, board.try_uncover(event)) {
                continue;
            }
            if board.tile_map.index(**event).is_bomb() {
                lost_event_writer.send(plugin_options.lost_event.clone());
            }
            if board.tile_map.bomb_count_at(**event) != 0 {
                continue;
            }
            board.tile_map.neighbour_coordinates(**event).for_each(|x| {
                events.push(TileUncoverEvent(x));
            });
            board.flagged.remove(event);
        }

        events.into_iter().for_each(|x| {
            tile_trigger_events.send(x);
        })
    }
}

#[inline(always)]
fn add_uncover_marker(commands: &mut Commands, entity: Option<Entity>) -> bool {
    if let Some(entity) = entity {
        commands.entity(entity).insert(UncoverMarker);
        return true;
    }
    false
}
