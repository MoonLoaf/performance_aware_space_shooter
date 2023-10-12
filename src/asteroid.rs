use specs::{System, WriteStorage, Join};

use crate::{components, render};
pub struct AsteroidMover;

impl<'a> System<'a> for AsteroidMover {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Asteroid>
    );

    fn run (&mut self, mut data: Self::SystemData) {
        for (pos, renderable, asteroid) in (&mut data.0, &mut data.1, &data.2).join() {
            let radians = pos.rot.to_radians();

            pos.x += asteroid.speed * radians.sin();
            pos.y -= asteroid.speed* radians.cos();

            let half_width = (renderable.output_width / 2) as u32;
            let half_height = (renderable.output_height / 2) as u32;

            if pos.x > (crate::SCREEN_WIDTH - half_width).into() || pos.x < half_width.into() {
                pos.rot = 360.0 - pos.rot;
            }
            else if pos.y > (crate::SCREEN_HEIGHT - half_height).into() || pos.y < half_height.into() {
                if pos.rot > 180.0 {
                    pos.rot = 540.0 - pos.rot;
                }
                else
                {
                    pos.rot = 180.0 - pos.rot;
                }
            }
            renderable.img_rotation += asteroid.rotation_speed;

            if renderable.img_rotation > 360.0 {
                renderable.img_rotation -= 360.0;
            }
            if renderable.img_rotation < 360.0 {
                renderable.img_rotation += 360.0;
            }
        }
    }
}
