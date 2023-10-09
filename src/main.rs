use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

fn main() -> Result<(), String> {
    println!("Starting");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem.window("Space Shooter | Oskar Wistedt", 800, 600)
        .position_centered()
        .build()
        .expect("Could not init video subsystem");

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
        //Cap the event pump loop to run 60 times per second
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
    return Ok(())
}