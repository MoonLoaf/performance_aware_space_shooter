use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};
use vector2d::Vector2D;
use rand::Rng;
use sdl2::libc::{commit, rewind};

use crate::components;
use crate::components::Laser;
use crate::key_manager;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {

    reload_world_if_no_players(ecs);


    let mut player_pos = components::Position { x: 0.0, y: 0.0, rot: 0.0 };
    let mut should_fire_laser = false;

    {
        let mut positions = ecs.write_storage::<crate::components::Position>();
        let mut player = ecs.write_storage::<crate::components::Player>();
        let mut renderables = ecs.write_storage::<crate::components::Renderable>();

        for (player, pos, renderable) in (&mut player, &mut positions, &mut renderables).join() {
            if crate::key_manager::is_key_pressed(&key_manager, "D") {
                pos.rot += player.rotation_speed;
            }
            if crate::key_manager::is_key_pressed(&key_manager, "A") {
                pos.rot -= player.rotation_speed;
            }

            update_movement(pos, player);
            if crate::key_manager::is_key_pressed(&key_manager, "W") {
                let radians = pos.rot.to_radians();

                let move_vec = Vector2D::<f64>::new(player.max_speed * radians.sin(),
                                                    player.max_speed * radians.cos());

                player.impulse += move_vec;
            }

            //Keep the player withing 360 degrees
            if pos.rot > 360.0 {
                pos.rot -= 360.0;
            }
            if pos.rot < 360.0 {
                pos.rot += 360.0;
            }

            //Screen wrapping
            if pos.x > crate::SCREEN_WIDTH.into() {
                pos.x -= crate::SCREEN_WIDTH as f64;
            }
            if pos.x < 0.0 {
                pos.x += crate::SCREEN_WIDTH as f64;
            }
            if pos.y > crate::SCREEN_HEIGHT.into() {
                pos.y -= crate::SCREEN_HEIGHT as f64;
            }
            if pos.y < 0.0 {
                pos.y += crate::SCREEN_HEIGHT as f64;
            }

            if key_manager::is_key_pressed(&key_manager, " ") {
                key_manager::key_up(key_manager, " ".to_string());
                should_fire_laser = true;
                player_pos.x = pos.x;
                player_pos.y = pos.y;
                player_pos.rot = pos.rot;
            }

            renderable.img_rotation = pos.rot;
        }
    }
    if should_fire_laser {
        fire_laser(ecs, player_pos);
    }
}

pub fn update_movement(pos: &mut crate::components::Position, player: &mut crate::components::Player) {

    player.current_speed *= player.friction;
    player.current_speed += player.impulse;

    if player.current_speed.length() > player.max_speed {
        player.current_speed = player.current_speed.normalise();
        player.current_speed = player.current_speed * player.max_speed;
    }

    pos.x += player.current_speed.x;
    pos.y -= player.current_speed.y;

    player.impulse = vector2d::Vector2D::new(0.0, 0.0);
}

pub fn load_world( ecs: &mut World) {
    //Create Player
    ecs.create_entity()
        .with(crate::components::Position { x: 350.0, y: 250.0, rot: 0.0 })
        .with(crate::components::Renderable {
            texture_name: String::from("Assets/Images/rocket.png"),
            img_width: 561,
            img_height: 644,
            output_width: 100,
            output_height: 140,
            img_rotation: 0.0
        })
        .with(crate::components::Player {
            impulse: vector2d::Vector2D::new(0.0,0.0,),
            current_speed: vector2d::Vector2D::new(0.0,0.0,),
            rotation_speed: 3.0,
            max_speed: 4.5,
            friction: 7.0
        })
    .build();
    //Asteroid
    ecs.create_entity()
        .with(crate::components::Position { x: 500.0, y: 235.0, rot: 45.0 })
        .with(crate::components::Renderable {
            texture_name: String::from(get_random_asteroid_texture_name()),
            img_width: 518,
            img_height: 517,
            output_width: 100,
            output_height: 100,
            img_rotation: 0.0
        })
        .with(crate::components::Asteroid{
            rotation_speed: 2.0,
            speed: 4.0,
            friction: 7.0
        })
    .build();
}

const MAX_LASERS: usize = 5;
fn fire_laser(ecs: &mut World, player_position: components::Position) {
    {
        let lasers = ecs.read_storage::<components::Laser>();
        if lasers.count() > MAX_LASERS - 1 {
            return;
        }
    }
    ecs.create_entity()
        .with(player_position)
        .with(components::Renderable {
            texture_name: String::from("Assets/Images/laser.png"),
            img_width: 605,
            img_height: 751,
            output_width: 60,
            output_height: 110,
            img_rotation: 0.0
         })
        .with(components::Laser {
            speed: 8.0
    })
    .build();
}

fn get_random_asteroid_texture_name() -> String {
    //TODO change to 1..=3 when asteroids have correct resolution
    let random_number = rand::thread_rng().gen_range(2..=3);
    format!("Assets/Images/asteroid_{}.png", random_number)
}

fn reload_world_if_no_players(ecs: &mut World) {
    let mut must_reload_world = false;
    {
        let players = ecs.read_storage::<crate::components::Player>();
        if players.join().count() < 1 {
            must_reload_world = true;
        }
    }
    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }
}