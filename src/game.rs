use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};
use vector2d::Vector2D;
use rand::Rng;

use crate::{components};
use crate::components::GameData;
use crate::input_manager;

#[derive(PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

const PLAYER_MAX_HEALTH: i32 = 10;
pub fn update(ecs: &mut World, input_manager: &mut HashMap<String, bool>, delta_time: f64) {
    reload_world_if_no_players(ecs);

    let mut current_player_pos = components::Position { x: 0.0, y: 0.0, rot: 0.0 };
    {
        let players = ecs.read_storage::<crate::components::Player>();
        let positions = ecs.read_storage::<crate::components::Position>();

        for (pos, _) in (&positions, &players).join() {
            current_player_pos.x = pos.x;
            current_player_pos.y = pos.y;
        }
    }
    {
        let asteroid_count;
        {
            let asteroids = ecs.read_storage::<crate::components::Asteroid>();
            asteroid_count = asteroids.join().count();
        }
        if asteroid_count < 1 {
            spawn_asteroids(ecs, &current_player_pos, false);
        }
    }

    let mut player_pos = components::Position { x: 0.0, y: 0.0, rot: 0.0 };
    let mut should_fire_laser = false;

    {
        let mut positions = ecs.write_storage::<components::Position>();
        let mut player = ecs.write_storage::<components::Player>();
        let mut renderables = ecs.write_storage::<components::Renderable>();

        for (player, pos, renderable) in (&mut player, &mut positions, &mut renderables).join() {
            if input_manager::is_key_pressed(&input_manager, "D") {
                pos.rot += player.rotation_speed * delta_time;
            }
            if input_manager::is_key_pressed(&input_manager, "A") {
                pos.rot -= player.rotation_speed * delta_time;
            }

            update_movement(pos, player, delta_time);
            if input_manager::is_key_pressed(&input_manager, "W") {
                let radians = pos.rot.to_radians();

                let move_vec = Vector2D::<f64>::new(player.max_speed * radians.sin(), player.max_speed * radians.cos());

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

            //Shooting
            if input_manager::is_key_pressed(&input_manager, " ") {
                input_manager::key_up(input_manager, " ".to_string());
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
    //toggle player invincibility
    if input_manager::is_key_pressed(&input_manager, "i") {
        input_manager::key_up(input_manager, "i".to_string());
        let mut gamedata = ecs.write_storage::<GameData>();
        for data in (&mut gamedata).join() {
            data.invincible_player = !data.invincible_player;
            //println!("invincible_player: {}", data.invincible_player);
        }
    }
    //spawning 1000 asteroids
    if input_manager::is_key_pressed(&input_manager, "o") {
        input_manager::key_up(input_manager, "o".to_string());
        spawn_asteroids(ecs, &current_player_pos, true);
    }
}

pub fn update_movement(pos: &mut components::Position, player: &mut components::Player, delta_time: f64) {

    player.current_speed *= player.friction;
    player.current_speed += player.impulse;

    if player.current_speed.length() > player.max_speed {
        player.current_speed = player.current_speed.normalise();
        player.current_speed = player.current_speed * player.max_speed;
    }

    pos.x += player.current_speed.x * delta_time;
    pos.y -= player.current_speed.y * delta_time;

    player.impulse = vector2d::Vector2D::new(0.0, 0.0);
}

pub fn load_world( ecs: &mut World) {
    //Create Player
    ecs.create_entity()
        .with(components::Position { x: 350.0, y: 250.0, rot: 0.0 })
        .with(components::Renderable {
            texture_name: String::from("Assets/Images/rocket.png"),
            img_width: 276,
            img_height: 364,
            output_width: 60,
            output_height: 80,
            img_rotation: 0.0
        })
        .with(components::Player {
            impulse: vector2d::Vector2D::new(0.0,0.0,),
            current_speed: vector2d::Vector2D::new(0.0,0.0,),
            rotation_speed: 200.0,
            max_speed: 200.0,
            friction: 0.9995,
            health: 3
        })
    .build();
    //Asteroid
    ecs.create_entity()
        .with(components::Position { x: 500.0, y: 235.0, rot: 45.0 })
        .with(components::Renderable {
            texture_name: String::from(get_random_asteroid_texture_name()),
            img_width: 215,
            img_height: 215,
            output_width: 100,
            output_height: 100,
            img_rotation: 0.0
        })
        .with(components::Asteroid{
            rotation_speed: 200.0,
            speed: 200.0,
            friction: 1.0
        })
    .build();

    ecs.create_entity()
        .with(components::GameData{score: 0, level: 1, invincible_player: false})
    .build();
}

const MAX_LASERS: usize = 200;
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
            speed: 500.0
    })
    .build();
}

fn spawn_asteroids(ecs: &mut World, player_pos: &components::Position, forced: bool) {
   if !forced {
       {
           let mut game_data = ecs.write_storage::<components::GameData>();
           for gamedata in (&mut game_data).join() {
               gamedata.level += 1;
           }
           let mut players = ecs.write_storage::<components::Player>();
           for player in (&mut players).join() {
               if player.health < PLAYER_MAX_HEALTH {
                   player.health += 1;
               }
           }
       }

       let amount_to_spawn: u32 = {
           let game_data = ecs.read_storage::<components::GameData>();
           let mut amount = 0;
           for data in (&game_data).join() {
               amount = data.level * 2;
               //amount = 10000;
           }
           amount
       };
       for _ in 0..amount_to_spawn {
           let spawn_position = generate_spawn_position(&player_pos);
           let asteroid_speed = rand::thread_rng().gen_range(70.0..250.0);
           let asteroid_rotation_speed = rand::thread_rng().gen_range(-400.0..400.0);
           let asteroid_size = rand::thread_rng().gen_range(40..110);

           create_asteroid(ecs, spawn_position, asteroid_size, asteroid_speed, asteroid_rotation_speed);
       }
   }
   else
   {
       for _ in 0..1000 {
           let spawn_position = generate_spawn_position(&player_pos);
           let asteroid_speed = rand::thread_rng().gen_range(70.0..250.0);
           let asteroid_rotation_speed = rand::thread_rng().gen_range(-400.0..400.0);
           let asteroid_size = rand::thread_rng().gen_range(40..110);

           create_asteroid(ecs, spawn_position, asteroid_size, asteroid_speed, asteroid_rotation_speed);
       }
   }
}

fn create_asteroid(ecs: &mut World, position: components::Position, asteroid_size: u32, asteroid_speed: f64, asteroid_rotation_speed: f64){
    // Calculate adjusted position to keep the entire asteroid within the screen bounds
    let half_size = asteroid_size as f64 / 2.0;
    let adjusted_x = position.x.max(half_size).min(crate::SCREEN_WIDTH as f64 - half_size);
    let adjusted_y = position.y.max(half_size).min(crate::SCREEN_HEIGHT as f64 - half_size);

    ecs.create_entity()
        .with(components::Position {
            x: adjusted_x,
            y: adjusted_y,
            rot: position.rot,
        })
        .with(components::Renderable {
            texture_name: String::from(get_random_asteroid_texture_name()),
            img_width: 215,
            img_height: 215,
            output_width: asteroid_size,
            output_height: asteroid_size,
            img_rotation: 0.0
        })
        .with(components::Asteroid{
            rotation_speed: asteroid_rotation_speed,
            speed: asteroid_speed,
            friction: 1.0
        })
    .build();
}

fn generate_spawn_position(player_pos: &components::Position) -> components::Position {
    let player_quadrant = get_player_quadrant(player_pos);

    // Determine quadrant based on rng. Can never be the player quadrant
    let random_chance: f64 = rand::thread_rng().gen_range(0.0..1.0);

    let asteroid_spawn_quadrant = match player_quadrant {
        Quadrant::TopLeft => {
            if random_chance < 0.5 {
                Quadrant::BottomRight
            } else if random_chance < 0.75 {
                Quadrant::TopRight
            } else {
                Quadrant::BottomLeft
            }
        },
        Quadrant::TopRight => {
            if random_chance < 0.5 {
                Quadrant::BottomLeft
            } else if random_chance < 0.75 {
                Quadrant::TopLeft
            } else {
                Quadrant::BottomRight
            }
        },
        Quadrant::BottomLeft => {
            if random_chance < 0.5 {
                Quadrant::TopRight
            } else if random_chance < 0.75 {
                Quadrant::BottomRight
            } else {
                Quadrant::TopLeft
            }
        },
        Quadrant::BottomRight => {
            if random_chance < 0.5 {
                Quadrant::TopLeft
            } else if random_chance < 0.75 {
                Quadrant::BottomLeft
            } else {
                Quadrant::TopRight
            }
        },
    };

    //Get random point within the selected part of the screen
    let (spawn_x, spawn_y) = match asteroid_spawn_quadrant {
        Quadrant::TopLeft => (
            rand::thread_rng().gen_range(0.0..crate::SCREEN_WIDTH as f64 / 2.0),
            rand::thread_rng().gen_range(0.0..crate::SCREEN_HEIGHT as f64 / 2.0),
        ),
        Quadrant::TopRight => (
            rand::thread_rng().gen_range(crate::SCREEN_WIDTH as f64 / 2.0..crate::SCREEN_WIDTH as f64),
            rand::thread_rng().gen_range(0.0..crate::SCREEN_HEIGHT as f64 / 2.0),
        ),
        Quadrant::BottomLeft => (
            rand::thread_rng().gen_range(0.0..crate::SCREEN_WIDTH as f64 / 2.0),
            rand::thread_rng().gen_range(crate::SCREEN_HEIGHT as f64 / 2.0..crate::SCREEN_HEIGHT as f64),
        ),
        Quadrant::BottomRight => (
            rand::thread_rng().gen_range(crate::SCREEN_WIDTH as f64 / 2.0..crate::SCREEN_WIDTH as f64),
            rand::thread_rng().gen_range(crate::SCREEN_HEIGHT as f64 / 2.0..crate::SCREEN_HEIGHT as f64),
        ),
    };

    let random_rotation = rand::thread_rng().gen_range(0.0..360.0);

    components::Position {
        x: spawn_x,
        y: spawn_y,
        rot: random_rotation,
    }
}

fn get_player_quadrant(pos: &components::Position) -> Quadrant {
    if pos.x < crate::SCREEN_WIDTH as f64 / 2.0 {
        if pos.y < crate::SCREEN_HEIGHT as f64 / 2.0 {
            //println!("Player is Top left");
            Quadrant::TopLeft
        } else {
            //println!("Player is Bot left");
            Quadrant::BottomLeft
        }
    } else {
        if pos.y < crate::SCREEN_HEIGHT as f64 / 2.0 {
            //println!("Player is Top Right");
            Quadrant::TopRight
        } else {
            //println!("Player is Bot Right");
            Quadrant::BottomRight
        }
    }
}

fn get_random_asteroid_texture_name() -> String {
    let random_number = rand::thread_rng().gen_range(1..=3);
    format!("Assets/Images/asteroid_{}.png", random_number)
}

fn reload_world_if_no_players(ecs: &mut World) {
    let mut must_reload_world = false;
    {
        let players = ecs.read_storage::<components::Player>();
        if players.join().count() < 1 {
            must_reload_world = true;
        }
    }
    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }
}