use log::info;
use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Sprite>,
                       WriteStorage<'a, MovementAnimation>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (vel, sprite, anim) in (&data.0, &mut data.1, &mut data.2).join() {
            if vel.speed == 0 {
                continue;
            }

            let frames = match vel.direction {
                Direction::Up => &anim.up_frames,
                Direction::Down => &anim.down_frames,
                Direction::Left => &anim.left_frames,
                Direction::Right => &anim.right_frames,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
         }
    }
}