use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::{InitFlag};

use specs::{World, WorldExt, Join, DispatcherBuilder};

use std::collections::HashMap;
use std::time::{Duration, Instant};

use texture_manager::TextureManager;

pub mod input_manager;
pub mod components;
pub mod game;
pub mod asteroid;
pub mod laser;
pub mod texture_manager;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

struct State { ecs: World }

#[derive(Default)]
pub struct DeltaTime(f64);

fn main() -> Result<(), String> {
    //println!("Starting");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("Space Shooter | Oskar Wistedt", 1920, 1080)
    .position_centered()
        .fullscreen()
        .build()
        .expect("Could not init video subsystem");

    let mut canvas = window.into_canvas().accelerated().build().expect("init canvas failed");
    let texture_creator = canvas.texture_creator();

    //Load and add these to texture manager
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("Assets/Images/rocket.png".to_string(), &texture_creator)?;
    texture_manager.add_texture("Assets/Images/asteroid_1.png".to_string(), &texture_creator)?;
    texture_manager.add_texture("Assets/Images/asteroid_2.png".to_string(), &texture_creator)?;
    texture_manager.add_texture("Assets/Images/asteroid_3.png".to_string(), &texture_creator)?;
    texture_manager.add_texture("Assets/Images/laser.png".to_string(), &texture_creator)?;

    
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(&"Assets/Fonts/Orbitron-Regular.ttf", 100)?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut input_manager: HashMap<String, bool> = HashMap::new();

    //ecs component registration
    let mut game_state = State { ecs: World::new() };
    game_state.ecs.register::<components::Position>();
    game_state.ecs.register::<components::Renderable>();
    game_state.ecs.register::<components::Player>();
    game_state.ecs.register::<components::Asteroid>();
    game_state.ecs.register::<components::Laser>();
    game_state.ecs.register::<components::GameData>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(asteroid::AsteroidMovement, "asteroid_movement", &[])
        //.with(asteroid::AsteroidCollider, "asteroid_collider", &[])
        .with(laser::LaserMovement, "laser_movement", &[])
        .with(laser::LaserDamage, "laser_damage", &[])
        .build();

    game::load_world(&mut game_state.ecs);

    game_state.ecs.insert(DeltaTime(0.0));

    let mut frame_count = 0u64;
    let mut last_frame_time = Instant::now();
    let mut last_frame_time_fps = Instant::now();
    let mut fps = 0u64;

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
                    input_manager::key_down(&mut input_manager, " ".to_string())
                },
                Event::KeyUp {keycode: Some(Keycode::Space), .. } => {
                    input_manager::key_up(&mut input_manager, " ".to_string())
                },
                //Keyboard events sent to input_manager
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            input_manager::key_down(&mut input_manager, key.to_string())
                        }
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            input_manager::key_up(&mut input_manager, key.to_string())
                        }
                    }
                }
                _ => {}
            }
        }
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f64();
        last_frame_time = now;

        frame_count += 1;

        let elapsed_time_fps = last_frame_time_fps.elapsed();
        if elapsed_time_fps.as_secs() >= 1 {
            fps = frame_count;
            frame_count = 0;
            last_frame_time_fps += Duration::new(1, 0);
        }

        // Update DeltaTime resource with the new value
        game_state.ecs.write_resource::<DeltaTime>().0 = delta_time;

        game::update(&mut game_state.ecs, &mut input_manager, delta_time);
        dispatcher.dispatch(&game_state.ecs);
        game_state.ecs.maintain();
        render(&mut canvas, &texture_creator, &mut texture_manager, &font, &game_state.ecs, &fps)?;
    }
    return Ok(())
}

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, texture_manager: &mut TextureManager, font: &sdl2::ttf::Font, ecs: &World, fps: &u64) -> Result<(), String> {

    let color = Color::RGB(0,0,0);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (renderable, pos) in (&renderables, &positions).join() {

        let src = Rect::new(0, 0, renderable.img_width, renderable.img_height);
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;

        let dest = Rect::new(x - ((renderable.output_width / 2) as i32), y - ((renderable.output_height / 2) as i32), renderable.output_width, renderable.output_height);
        let center = Point::new((renderable.output_width / 2) as i32, (renderable.output_height / 2) as i32);

        let texture = texture_manager.get_texture(&renderable.texture_name).ok_or("Texture not found")?;
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

    //Health
    let players = ecs.read_storage::<components::Player>();
    for player in (&players).join() {
        let health_text = "Health: ".to_string() + &player.health.to_string();

        let surface = font.render(&health_text).solid(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

        let target = Rect::new((crate::SCREEN_WIDTH - 290) as i32, 0i32, 110u32, 50u32);
        canvas.copy(&surface_texture, None, Some(target))?;
    }

    let game_data = ecs.read_storage::<components::GameData>();
    for game_data in (&game_data).join() {
        //Score
        let score_text = "Score: ".to_string() + &game_data.score.to_string();

        let surface = font.render(&score_text).solid(Color::RGB(255, 255, 255)).map_err(|e|e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

        let target = Rect::new(10i32, 0i32, 140u32, 50u32);
        canvas.copy(&surface_texture, None, Some(target))?;

        //Level
        let level_text = "Level: ".to_string() + &game_data.level.to_string();

        let surface = font.render(&level_text).solid(Color::RGB(255,255,255)).map_err(|e|e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

        let target = Rect::new((crate::SCREEN_WIDTH-140) as i32, 0i32, 110u32, 50u32);
        canvas.copy(&surface_texture, None, Some(target))?;
    }
    //Total entities
    {
        let entity_count = ecs.entities().join().count();
        let entity_text = "Total Entities: ".to_string() + &entity_count.to_string();

        let surface = font.render(&entity_text).solid(Color::RGB(255, 0, 0)).map_err(|e| e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

        let target = Rect::new(10i32, (crate::SCREEN_HEIGHT - 100) as i32, 150u32, 60u32);
        canvas.copy(&surface_texture, None, Some(target))?;
    }
    //fps
    {
        let fps_text = "fps: ".to_string() + &fps.to_string();

        let surface = font.render(&fps_text).solid(Color::RGB(0, 255, 0)).map_err(|e| e.to_string())?;
        let surface_texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

        let target = Rect::new((crate::SCREEN_WIDTH-140) as i32, (crate::SCREEN_HEIGHT - 100) as i32, 90u32, 40u32);
        canvas.copy(&surface_texture, None, Some(target))?;
    }

    canvas.present();
    Ok(())
}