use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::image::{self, InitFlag, LoadTexture};

use specs::{World, WorldExt, Join};

use std::path::Path;
use std::collections::HashMap;
use std::time::Duration;

use rand::*;

pub mod key_manager;
pub mod components;
pub mod game;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

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
    
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(&"Assets/Fonts/Orbitron-Regular.ttf", 100)?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    //ecs component registration
    let mut game_state = State { ecs: World::new() };
    game_state.ecs.register::<components::Position>();
    game_state.ecs.register::<components::Renderable>();
    game_state.ecs.register::<components::Player>();

    game::load_world(&mut game_state.ecs);

    //Character
    let spaceship_texture = texture_creator.load_texture("Assets/Images/rocket.png")?;
    
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
        render(&mut canvas, &texture_creator, &font, &spaceship_texture, &game_state.ecs)?;
        
        //Cap the event pump loop to run 60 times per second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
    return Ok(())
}

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font, texture: &Texture, ecs: &World) -> Result<(), String> {

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

        canvas.copy_ex(
            &texture,
            src,
            dest,
            pos.rot,
            center,
            false,
            false
        )?;
    }

    canvas.present();
    Ok(())
}