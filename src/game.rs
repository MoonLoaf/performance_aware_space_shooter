use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};
use vector2d::Vector2D;
use rand::Rng;

use crate::components;
use crate::key_manager;

const SPAWN_DISTANCE: f64 = 400.0;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {

    reload_world_if_no_players(ecs);

    let mut current_player_pos = components::Position { x: 0.0, y: 0.0, rot: 0.0 };
    {
        let players = ecs.read_storage::<crate::components::Player>();
        let positions = ecs.read_storage::<crate::components::Position>();

        for (pos, player) in (&positions, &players).join() {
            current_player_pos.x = pos.x;
            current_player_pos.y = pos.y;
        }
    }

    let mut player_pos = components::Position { x: 0.0, y: 0.0, rot: 0.0 };
    let mut should_fire_laser = false;
    let mut should_create_asteroid = false;
    {
        let asteroids = ecs.read_storage::<crate::components::Asteroid>();
        if asteroids.join().count() < 1 {
            should_create_asteroid = true;
        }
    }

    if should_create_asteroid {
        let spawn_position = generate_spawn_position(&current_player_pos);

        let asteroid_speed = rand::thread_rng().gen_range(1.0..6.0);
        let asteroid_rotation_speed = rand::thread_rng().gen_range(1.0..5.0);
        let asteroid_size = rand::thread_rng().gen_range(40..110);

        create_asteroid(ecs, spawn_position, asteroid_size, asteroid_rotation_speed, asteroid_rotation_speed);
    }

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
            img_width: 276,
            img_height: 364,
            output_width: 60,
            output_height: 80,
            img_rotation: 0.0
        })
        .with(crate::components::Player {
            impulse: vector2d::Vector2D::new(0.0,0.0,),
            current_speed: vector2d::Vector2D::new(0.0,0.0,),
            rotation_speed: 3.0,
            max_speed: 4.0,
            friction: 5.0
        })
    .build();
    //Asteroid
    ecs.create_entity()
        .with(crate::components::Position { x: 500.0, y: 235.0, rot: 45.0 })
        .with(crate::components::Renderable {
            texture_name: String::from(get_random_asteroid_texture_name()),
            img_width: 215,
            img_height: 215,
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

const MAX_LASERS: usize = 7;
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
            img_width: 64,
            img_height: 153,
            output_width: 20,
            output_height: 50,
            img_rotation: 0.0
         })
        .with(components::Laser {
            speed: 10.0
    })
    .build();
}

fn create_asteroid(ecs: &mut World, position: components::Position, asteroid_size: u32, asteroid_speed: f64, asteroid_rotation_speed: f64){
    ecs.create_entity()
        .with(position)
        .with(crate::components::Renderable {
            texture_name: String::from(get_random_asteroid_texture_name()),
            img_width: 215,
            img_height: 215,
            output_width: asteroid_size,
            output_height: asteroid_size,
            img_rotation: 0.0
        })
        .with(crate::components::Asteroid{
            rotation_speed: asteroid_rotation_speed,
            speed: asteroid_speed,
            friction: 7.0
        })
    .build();
}

fn generate_spawn_position(player_pos: &components::Position) -> components::Position {
    let angle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
    let spawn_x = player_pos.x + (angle.cos() * SPAWN_DISTANCE);
    let spawn_y = player_pos.y + (angle.sin() * SPAWN_DISTANCE);

    // Generate random rotation in degrees
    let random_rotation = rand::thread_rng().gen_range(0.0..360.0);

    // Clamp within screen bounds
    let clamped_x = spawn_x.max(0.0).min(crate::SCREEN_WIDTH as f64);
    let clamped_y = spawn_y.max(0.0).min(crate::SCREEN_HEIGHT as f64);

    components::Position {
        x: clamped_x,
        y: clamped_y,
        rot: random_rotation
    }
}
fn get_random_asteroid_texture_name() -> String {
    let random_number = rand::thread_rng().gen_range(1..=3);
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