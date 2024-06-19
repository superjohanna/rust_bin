use bevy::prelude::*;

/// Can't pass the plugin as &self so I need to make a resource which holds all the data
#[derive(Debug, Clone, Resource)]
pub struct PluginOptions<TyRunState, TyPauseState, TyResetEvent, TyWonEvent, TyLostEvent>
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
