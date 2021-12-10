use sdl2::rect::{Point, Rect};
use specs::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct KeyboardControlled;

impl Component for KeyboardControlled {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct Position(pub Point);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

impl Component for MovementAnimation {
    type Storage = VecStorage<Self>;
}