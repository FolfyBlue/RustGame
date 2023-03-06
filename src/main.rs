extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
struct Player {
    x: i32,
    y: i32,
    rect: Rect
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("RustGame", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
    .into_canvas()
    .build()
    .map_err(|e| e.to_string())?;

    // Create a player
    let mut player = Player {
        x: 0,
        y: 0,
        rect: Rect::new(0, 0, 64, 64)
    };

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        
        // Draw white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        // Draw black player
        player.rect.set_x(player.x);
        player.rect.set_y(player.y);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(player.rect);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}