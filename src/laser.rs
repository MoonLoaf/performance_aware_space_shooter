use specs::prelude::*;
use specs::{Entities, Join};

use crate::components;

pub struct LaserMovement;

impl<'a> System<'a> for LaserMovement {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Laser>,
        Entities<'a>
    );
    fn run (&mut self, data: Self::SystemData) {
        let (mut positions, mut renderables, laser, entities) = data;

        for (position, renderable, laser, entity) in (&mut positions, &mut renderables, &laser, &entities).join() {
            let radians = position.rot.to_radians();

            position.x += laser.speed * radians.sin();
            position.y -= laser.speed * radians.cos();

            if position.x > crate::SCREEN_WIDTH.into() || position.x < 0.0 || position.y > crate::SCREEN_HEIGHT.into() || position.y < 0.0 {
                entities.delete(entity).ok();
            }

            renderable.img_rotation = position.rot;
        }
    }
}