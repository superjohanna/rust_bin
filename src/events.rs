use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Event)]
pub struct GameWonEvent;
#[derive(Debug, Clone, Copy, Event)]
pub struct GameLostEvent;

#[derive(Debug, Clone, Copy, Event)]
pub struct GameResetEvent;
