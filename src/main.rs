extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const GRAVITY: f64 = 0.001;
const MAX_VELOCITY: f64 = 1.0;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const PLAYER_SIZE: u32 = 64;
struct Player {
    x: i32,
    y: i32,
    velocity_x: f64,
    velocity_y: f64,
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

    let timer = sdl_context.timer()?;

    // Create a player
    let mut player = Player {
        x: (WIDTH/2 - PLAYER_SIZE/2) as i32,
        y: 0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        rect: Rect::new(0, 0, PLAYER_SIZE, PLAYER_SIZE)
    };

    let mut event_pump = sdl_context.event_pump()?;

    let mut old_ticks: i32 = 0;

    'running: loop {
        //- EVENT LOOP
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

        //- GAME LOGIC

        let new_ticks = timer.ticks() as i32;
        let delta_time = new_ticks - old_ticks;

        // Ensure the player doesn't surpass the maximum fall velocity
        if player.velocity_y <= MAX_VELOCITY {
            player.velocity_y = player.velocity_y + GRAVITY * (delta_time) as f64;
        } else {
            player.velocity_y = MAX_VELOCITY;
        }
        player.velocity_x = player.velocity_x * (delta_time) as f64;

        // Update the player position according to the given velocity
        player.x = player.x + (player.velocity_x.round() as i32)*delta_time;
        player.y = player.y + (player.velocity_y.round() as i32)*delta_time;

        // Ensure the player stays in bounds
        if player.y >= (HEIGHT - PLAYER_SIZE) as i32 {
            player.y = (HEIGHT - PLAYER_SIZE) as i32;
            player.velocity_y = 0.0;
        }
        
        print!("\x1B[2J\x1B[1;1H");
        println!("== PLAYER ==");
        println!("Position: {} {}", player.x, player.y);
        println!("Velocity: {} {}", player.velocity_x, player.velocity_y);
        
        //- DRAW

        // Draw white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        // Draw black player
        player.rect.set_x(player.x);
        player.rect.set_y(player.y);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(player.rect);

        canvas.present();
        old_ticks = new_ticks;
    }

    Ok(())
}