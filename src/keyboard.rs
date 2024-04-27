use bevy_ecs::system::{Query, Res, ResMut, Resource};
use sdl2::{event::Event, keyboard::Keycode};
use crate::{components::*, WorldState};

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Resource)]
pub struct KeyboardEvent(pub Option<Event>);

pub fn handle_keyboard_arrows(event: Res<KeyboardEvent>, mut query: Query<&mut Velocity>) {
  for mut velocity in &mut query {
    match event.0 {
      Some(Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. }) => {
        velocity.speed = PLAYER_MOVEMENT_SPEED;
        velocity.direction = Direction::Left;
      },
      Some(Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. }) => {
        velocity.speed = PLAYER_MOVEMENT_SPEED;
        velocity.direction = Direction::Right;
      },
      Some(Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. }) => {
        velocity.speed = PLAYER_MOVEMENT_SPEED;
        velocity.direction = Direction::Up;
      },
      Some(Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. }) => {
        velocity.speed = PLAYER_MOVEMENT_SPEED;
        velocity.direction = Direction::Down;
      },
      Some(Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. }) |
      Some(Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. }) |
      Some(Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. }) |
      Some(Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. }) => {
        velocity.speed = 0;
      },
      _ => ()
    }
  }
}

pub fn handle_quit(event: Res<KeyboardEvent>, mut state: ResMut<WorldState>) {
  match event.0 {
    Some(Event::Quit {..}) |
    Some(Event::KeyDown { keycode: Some(Keycode::Escape), .. }) => {
      state.run = false;
    },
    _ => ()
  }
}
