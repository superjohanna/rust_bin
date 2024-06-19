use bevy::prelude::Window;

pub(crate) fn adaptive_tile_size(
    window: &Window,
    (min, max): (f32, f32),
    (width, height): (u16, u16),
) -> f32 {
    let max_width = window.width() / width as f32;
    let max_height = window.height() / height as f32;
    max_width.min(max_height).clamp(min, max)
}
