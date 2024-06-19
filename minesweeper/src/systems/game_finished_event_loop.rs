use std::ops::Index;

use bevy::prelude::*;

use crate::{
    resources::{board::Board, plugin_options::PluginOptions},
    MinesweeperPlugin, TileFlagEvent, TileUncoverEvent,
};

impl<
        TyRunState: States + PartialEq,
        TyPauseState: States + PartialEq,
        TyResetEvent: Event + Clone,
        TyWonEvent: Event + Clone,
        TyLostEvent: Event + Clone,
    > MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
{
    pub(crate) fn game_end_loop(
        plugin_options: Res<
            PluginOptions<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>,
        >,
        mut tile_flag_event_reader: EventReader<TileFlagEvent>,
        mut tile_uncover_event_reader: EventReader<TileUncoverEvent>,
        mut won_event_writer: EventWriter<TyWonEvent>,
        board: Res<Board>,
    ) {
        if board.flag_count != *board.tile_map.bomb_count()
            || (tile_flag_event_reader.read().count() == 0
                && tile_uncover_event_reader.read().count() == 0)
        {
            return;
        }

        // Check if all flags are placed correctly
        let mut all_correct_flags = true;
        for (coord, (_, _, is_flagged)) in board.flagged.iter() {
            if !is_flagged {
                continue;
            }
            if !board.tile_map.index(*coord).is_bomb() {
                all_correct_flags = false;
                break;
            }
        }

        // Check if all non bombs tiles are revealed
        let mut all_correct_covered = true;
        for (coord, _) in board.covered.iter() {
            if !board.tile_map.index(*coord).is_bomb() {
                all_correct_covered = false;
                break;
            }
        }

        if all_correct_covered || all_correct_flags {
            won_event_writer.send(plugin_options.won_event.clone());
        }
    }
}
