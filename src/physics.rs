use bevy_ecs::system::Query;

use crate::components::{Direction, Position, Velocity};

pub fn move_position(mut query: Query<(&mut Position, &Velocity)>) {
  // TODO: This code can be made nicer and more idiomatic using more pattern matching.
  // Look up "rust irrefutable patterns" and use them here.
  for (mut position, velocity) in &mut query {
    match velocity.direction {
      Direction::Left => {
        position.0 = position.0.offset(-velocity.speed, 0);
      },
      Direction::Right => {
        position.0 = position.0.offset(velocity.speed, 0);
      },
      Direction::Up => {
        position.0 = position.0.offset(0, -velocity.speed);
      },
      Direction::Down => {
        position.0 = position.0.offset(0, velocity.speed);
      },
    }
  }
}
