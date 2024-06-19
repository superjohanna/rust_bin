pub(crate) mod board;
pub(crate) mod components;
pub mod events;
pub(crate) mod helpers;
pub mod options;
pub(crate) mod resources;
pub(crate) mod states;
pub(crate) mod systems;
pub mod texture_handles;

use bevy::prelude::*;

use crate::{
    events::{TileFlagEvent, TileUncoverEvent},
    resources::plugin_options::PluginOptions,
    states::plugin_state::MinesweeperState,
};

pub(crate) const BACKGROUND_Z: f32 = 0.;
pub(crate) const FOREGROUND_Z: f32 = 1.;

pub struct MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
where
    TyRunState: States + PartialEq,
    TyPauseState: States + PartialEq,
    TyResetEvent: Event + Clone,
    TyWonEvent: Event + Clone,
    TyLostEvent: Event + Clone,
{
    pub run_state: TyRunState,
    pub pause_state: TyPauseState,
    pub reset_event: TyResetEvent,
    pub won_event: TyWonEvent,
    pub lost_event: TyLostEvent,
}

impl<
        TyRunState: States + PartialEq,
        TyPauseState: States + PartialEq,
        TyResetEvent: Event + Clone,
        TyWonEvent: Event + Clone,
        TyLostEvent: Event + Clone,
    > Plugin
    for MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
{
    /// Events need to be already added to the engine!
    fn build(&self, app: &mut App) {
        app.init_state::<MinesweeperState>()
            .add_event::<TileUncoverEvent>()
            .add_event::<TileFlagEvent>()
            .insert_resource(PluginOptions {
                run_state: self.run_state.clone(),
                pause_state: self.pause_state.clone(),
                reset_event: self.reset_event.clone(),
                won_event: self.won_event.clone(),
                lost_event: self.lost_event.clone(),
            })
            .add_systems(
                Update,
                (
                    Self::create_board
                        .run_if(in_state(MinesweeperState::NewGame))
                        .run_if(in_state(self.run_state.clone())),
                    crate::systems::rescale::rescale,
                    crate::systems::input::input.run_if(
                        in_state(MinesweeperState::Running)
                            .and_then(in_state(self.run_state.clone()))
                            .and_then(not(in_state(self.pause_state.clone()))),
                    ),
                    crate::systems::remove_cover::remove_cover
                        .run_if(in_state(MinesweeperState::Running)),
                    Self::tile_uncover_event_handler.run_if(in_state(MinesweeperState::Running)),
                    crate::systems::tile_flag_event_handler::tile_flag_event_handler
                        .run_if(in_state(MinesweeperState::Running)),
                    Self::game_end_loop.run_if(in_state(MinesweeperState::Running)),
                    Self::reset_event_handler,
                ),
            );
        bevy::log::info!("Plugin loaded.");
    }
}

impl<
        TyRunState: States + PartialEq,
        TyPauseState: States + PartialEq,
        TyResetEvent: Event + Clone,
        TyWonEvent: Event + Clone,
        TyLostEvent: Event + Clone,
    > MinesweeperPlugin<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
{
    pub fn new(
        run_state: TyRunState,
        pause_state: TyPauseState,
        reset_event: TyResetEvent,
        won_event: TyWonEvent,
        lost_event: TyLostEvent,
    ) -> Self {
        Self {
            run_state,
            pause_state,
            reset_event,
            won_event,
            lost_event,
        }
    }
}
