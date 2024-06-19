use bevy::{ecs::query::QueryFilter, prelude::*};

#[derive(Debug, Clone, Copy, Component, QueryFilter)]
pub struct BoardMarker;
