mod components;
mod keyboard;
mod physics;
mod player;
mod renderer;

use bevy_ecs::system::Resource;
use components::{Color, Direction, KeyboardControlled, Position, Size, Velocity};
use keyboard::{handle_keyboard_arrows, handle_quit, KeyboardEvent};
use physics::move_position;
use player::PlayerBundle;
use renderer::render_entities;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Point;
use std::time::Duration;
use bevy_ecs::world::World;
use bevy_ecs::prelude::Schedule;

#[derive(Resource)]
struct WorldState {
  run: bool,
}

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem.window("game tutorial", 800, 600)
    .position_centered()
    .build()
    .expect("could not initialize video subsystem");

  let mut canvas = window.into_canvas().build()
    .expect("could not make a canvas");

  let mut world = World::default();
  let world_state = WorldState {
    run: true
  };

  world.insert_resource(KeyboardEvent(None));
  world.insert_resource(world_state);

  let player = world.spawn(PlayerBundle {
    keyboard_controlled: KeyboardControlled,
    position: Position(Point::new(50, 50)),
    velocity: Velocity { speed: 0, direction: Direction::Right },
    size: Size(20),
    color: Color(SdlColor::RGB(255, 0, 0)),
  }).id();

  let mut schedule = Schedule::default();
  schedule
    .add_systems(handle_quit)
    .add_systems(handle_keyboard_arrows)
    .add_systems(move_position);

  let mut event_pump = sdl_context.event_pump()?;
  'running: loop {
    if !world.resource::<WorldState>().run {
      break 'running;
    }
    // Handle events
    for event in event_pump.poll_iter() {
      if event.is_keyboard() {
        world.insert_resource(KeyboardEvent(Some(event.clone())));
      }
    }

    schedule.run(&mut world);

    render_entities(
      &mut canvas,
      world.get::<Position>(player).unwrap(),
      world.get::<Color>(player).unwrap(),
      world.get::<Size>(player).unwrap()
    );

    // Time management!
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }

  Ok(())
}
