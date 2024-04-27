use bevy_ecs::bundle::Bundle;

use crate::components::{Color, KeyboardControlled, Position, Size, Velocity};

#[derive(Bundle)]
pub struct PlayerBundle {
  pub keyboard_controlled: KeyboardControlled,
  pub position: Position,
  pub velocity: Velocity,
  pub color: Color,
  pub size: Size
}
