use bevy::ecs::{component::Component, query::QueryFilter};

#[derive(Debug, Clone, Copy, Component, QueryFilter)]
pub struct TileCoverMarker;
