use specs::prelude::*;
use specs::{Entities, Join};

use crate::{components};

pub struct LaserMovement;

impl<'a> System<'a> for LaserMovement {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Laser>,
        Entities<'a>,
        Read<'a, crate::DeltaTime>,
    );
    fn run (&mut self, data: Self::SystemData) {
        let (mut positions, mut renderables, laser, entities, delta_time) = data;

        for (position, renderable, laser, entity) in (&mut positions, &mut renderables, &laser, &entities).join() {
            let radians = position.rot.to_radians();

            position.x += laser.speed * delta_time.0 * radians.sin();
            position.y -= laser.speed * delta_time.0 * radians.cos();

            if position.x > crate::SCREEN_WIDTH.into() || position.x < 0.0 || position.y > crate::SCREEN_HEIGHT.into() || position.y < 0.0 {
                entities.delete(entity).ok();
            }

            renderable.img_rotation = position.rot;
        }
    }
}

pub struct LaserDamage;

impl<'a> System<'a> for LaserDamage {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Laser>,
        WriteStorage<'a, components::Asteroid>,
        WriteStorage<'a, components::GameData>,
        Entities<'a>
    );
    fn run (&mut self, data: Self::SystemData) {
        let (positions, renderables, lasers, asteroids, _ ,  entities) = &data;

        let mut should_add_score = false;

        for (laser_pos,_ ,_, laser_entity) in (positions, renderables, lasers, entities).join() {
            for (asteroid_pos, asteroid_renderable, _, asteroid_entity) in (positions, renderables, asteroids, entities).join() {
                let diff_x: f64 = (laser_pos.x - asteroid_pos.x).abs();
                let diff_y: f64 = (laser_pos.y - asteroid_pos.y).abs();

                let hypotenuse: f64 = ((diff_x * diff_x) + (diff_y * diff_y)).sqrt();

                if hypotenuse < asteroid_renderable.output_width as f64 / 2.0 {
                    entities.delete(laser_entity).ok();
                    entities.delete(asteroid_entity).ok();
                    should_add_score = true;
                }
            }
        }
        if should_add_score {
            let (_, _, _, _, mut gamedatas, _) = data;
            for gamedata in (&mut gamedatas).join() {
                gamedata.score += 10 * gamedata.level;
            }
        }
    }
}