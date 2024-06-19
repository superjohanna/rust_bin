use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Bounds2 {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.origin.x
            && coords.y >= self.origin.y
            && coords.x <= self.origin.x + self.size.y
            && coords.y <= self.origin.y + self.size.y
    }
}
