use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};
use vector2d::Vector2D;

pub fn update(ecs: &mut World, key_manager: &HashMap<String, bool>) {

    let mut positions = ecs.write_storage::<crate::components::Position>();
    let mut player = ecs.write_storage::<crate::components::Player>();

    for(player, pos) in (&mut player, &mut positions).join() {
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
    ecs.create_entity()
        .with(crate::components::Position { x: 350.0, y: 250.0, rot: 0.0 })
        .with(crate::components::Renderable {
            texture_name: String::from("Assets/Images/rocket.png"),
            img_width: 55,
            img_height: 77,
            output_width: 55,
            output_height: 77,
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
}