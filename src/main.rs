use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

use std::path::Path;
use std::time::Duration;

use rand::*;

fn main() -> Result<(), String> {
    println!("Starting");
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem.window("Space Shooter | Oskar Wistedt", 800, 600)
    .position_centered()
        .build()
        .expect("Could not init video subsystem");

    let mut canvas = window.into_canvas().build().expect("init canvas failed");
    let texture_creator = canvas.texture_creator();
    
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(&"Assets/Fonts/Orbitron-Regular.ttf", 100)?;
    
    let mut event_pump = sdl_context.event_pump()?;
    
    'running:loop {
        for event in event_pump.poll_iter() {

            match event {

                Event::Quit {..} =>{
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        render(&mut canvas, &texture_creator, &font);
        
        //Cap the event pump loop to run 60 times per second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
    return Ok(())
}

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font) -> Result<(), String> {
    let color = Color::RGB(0,0,50);
    canvas.set_draw_color(color);

    let test_text: String = "Testing".to_string();

    let surface = font.render(&test_text)
        .blended(Color::RGB(0, 255, 0))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);

    canvas.copy(&texture, None, Some(target))?;

    canvas.present();
    Ok(())
}