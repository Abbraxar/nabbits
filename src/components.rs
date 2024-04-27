use bevy_ecs::prelude::*;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct KeyboardControlled;

#[derive(Component, Debug)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
pub struct Velocity {
  pub speed: i32,
  pub direction: Direction,

}

#[derive(Component, Debug)]
pub struct Size(pub i32);

#[derive(Component, Debug)]
pub struct Color(pub SdlColor);
