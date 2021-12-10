use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (ReadStorage<'a, Velocity>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (vel, pos) in (&data.0, &mut data.1).join() {
                match vel.direction {
                    Direction::Up => {
                        pos.0 = pos.0.offset(0, -vel.speed);
                    },
                    Direction::Down => {
                        pos.0 = pos.0.offset(0, vel.speed);
                    },
                    Direction::Left => {
                        pos.0 = pos.0.offset(-vel.speed, 0);
                    },
                    Direction::Right => {
                        pos.0 = pos.0.offset(vel.speed, 0);
                    },
                }
         }
    }
}