use bevy::prelude::*;

pub struct RGB;

impl RGB {
    pub fn new((r, g, b): (u32, u32, u32)) -> Color {
        let upper_limit: f32 = 255.0;

        let r = r as f32 / upper_limit;
        let g = g as f32 / upper_limit;
        let b = b as f32 / upper_limit;

        Color::rgb(r, g, b)
    }
}

#[derive(Component)]
pub struct Size {
  pub width: f32,
  pub height: f32,
}

impl Size {
  pub const fn square(x: f32) -> Self {
      Self {
          width: x,
          height: x,
      }
  }
}