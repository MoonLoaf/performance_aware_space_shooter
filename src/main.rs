use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::image::{self, InitFlag, LoadTexture};

use specs::{World, WorldExt, Join, Dispatcher, DispatcherBuilder};

use std::path::Path;
use std::collections::HashMap;
use std::time::Duration;

use rand::*;

use texture_manager::TextureManager;

pub mod key_manager;
pub mod components;
pub mod game;
pub mod asteroid;
pub mod laser;
pub mod texture_manager;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

struct State { ecs: World }

fn main() -> Result<(), String> {
    println!("Starting");
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("Space Shooter | Oskar Wistedt", 1280, 720)
    .position_centered()
        .build()
        .expect("Could not init video subsystem");

    let mut canvas = window.into_canvas().build().expect("init canvas failed");
    let texture_creator = canvas.texture_creator();

    //Character texture
    let rocket_tex = texture_creator.load_texture("Assets/Images/rocket.png")?;
    let laser_tex = texture_creator.load_texture("Assets/Images/laser.png")?;
    //Asteroid textures
    let asteroid_1_tex = texture_creator.load_texture("Assets/Images/asteroid_1.png")?;
    let asteroid_2_tex = texture_creator.load_texture("Assets/Images/asteroid_2.png")?;
    let asteroid_3_tex = texture_creator.load_texture("Assets/Images/asteroid_3.png")?;

    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("Assets/Images/rocket.png".to_string(), &rocket_tex);
    texture_manager.add_texture("Assets/Images/asteroid_1.png".to_string(), &asteroid_1_tex);
    texture_manager.add_texture("Assets/Images/asteroid_2.png".to_string(), &asteroid_2_tex);
    texture_manager.add_texture("Assets/Images/asteroid_3.png".to_string(), &asteroid_3_tex);
    texture_manager.add_texture("Assets/Images/laser.png".to_string(), &laser_tex);

    
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(&"Assets/Fonts/Orbitron-Regular.ttf", 100)?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    //ecs component registration
    let mut game_state = State { ecs: World::new() };
    game_state.ecs.register::<components::Position>();
    game_state.ecs.register::<components::Renderable>();
    game_state.ecs.register::<components::Player>();
    game_state.ecs.register::<components::Asteroid>();
    game_state.ecs.register::<components::Laser>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(asteroid::AsteroidCollider, "asteroid_collider", &[])
        .with(laser::LaserMovement, "laser_movement", &[])
        .build();



    game::load_world(&mut game_state.ecs);

    'running:loop {
        for event in event_pump.poll_iter() {

            match event {
                //Application quit
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                //Shooting
                Event::KeyDown {keycode: Some(Keycode::Space), .. } => {
                    key_manager::key_down(&mut key_manager, " ".to_string())
                },
                Event::KeyUp {keycode: Some(Keycode::Space), .. } => {
                    key_manager::key_up(&mut key_manager, " ".to_string())
                },
                //Keyboard events sent to key_manager
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            key_manager::key_down(&mut key_manager, key.to_string())
                        }
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            key_manager::key_up(&mut key_manager, key.to_string())
                        }
                    }
                }
                _ => {}
            }
        }
        game::update(&mut game_state.ecs, &mut key_manager);
        dispatcher.dispatch(&game_state.ecs);
        game_state.ecs.maintain();
        render(&mut canvas, &texture_creator, &mut texture_manager, &font, &game_state.ecs)?;
        
        //Cap the event pump loop to run 60 times per second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
    return Ok(())
}

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, texture_manager: &mut TextureManager, font: &sdl2::ttf::Font, ecs: &World) -> Result<(), String> {

    let color = Color::RGB(0,0,0);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (mut renderable, pos) in (&renderables, &positions).join() {

        //TODO clean up amount of new variable declarations for each iteration of render()?

        let src = Rect::new(0, 0, renderable.img_width, renderable.img_height);
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;

        let dest = Rect::new(x - ((renderable.output_width / 2) as i32), y - ((renderable.output_height / 2) as i32), renderable.output_width, renderable.output_height);
        let center = Point::new((renderable.output_width / 2) as i32, (renderable.output_height / 2) as i32);

        let texture = texture_manager::TextureManager::get_texture(texture_manager, &renderable.texture_name).ok_or("Texture not found".to_string())?;
        canvas.copy_ex(
            &texture,
            src,
            dest,
            renderable.img_rotation,
            center,
            false,
            false
        )?;
    }

    canvas.present();
    Ok(())
}