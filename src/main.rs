extern crate sdl2;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const GRAVITY: f64 = 0.001;
const MAX_VELOCITY: f64 = 1.0;
const JUMP_VELOCITY: f64 = -0.6;
const MOVE_SPEED: f64 = 0.6;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const PLAYER_SIZE: u32 = 64;
struct Player {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    rect: Rect,
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

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let timer = sdl_context.timer()?;

    // Create a player
    let mut player = Player {
        x: (WIDTH / 2 - PLAYER_SIZE / 2) as f64,
        y: 0.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        rect: Rect::new(0, 0, PLAYER_SIZE, PLAYER_SIZE),
    };

    let mut event_pump = sdl_context.event_pump()?;

    let mut old_ticks: i32 = 0;

    let mut touching_ground: bool = false;

    'running: loop {
        //- VARIABLES

        let new_ticks = timer.ticks() as i32;
        let delta_time = new_ticks - old_ticks;
        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

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

        // Handle movement
        if keys.contains(&Keycode::Left) {
            player.velocity_x = -MOVE_SPEED * (delta_time) as f64;
            player.x += -MOVE_SPEED * (delta_time) as f64;
        }
        if keys.contains(&Keycode::Right) {
            player.velocity_x = MOVE_SPEED * (delta_time) as f64;
            player.x += MOVE_SPEED * (delta_time) as f64;
        }
        if keys.contains(&Keycode::Up) && touching_ground {
            player.velocity_y = JUMP_VELOCITY * (delta_time) as f64;
        }

        // Ensure the player doesn't surpass the maximum fall velocity
        if player.velocity_y <= MAX_VELOCITY {
            player.velocity_y = player.velocity_y + GRAVITY * (delta_time) as f64;
        } else {
            player.velocity_y = MAX_VELOCITY;
        }
        player.velocity_x = player.velocity_x * (delta_time) as f64;

        // Update the player position according to the given velocity
        player.x = player.x + player.velocity_x * (delta_time as f64);
        player.y = player.y + player.velocity_y * (delta_time as f64);

        // Ensure the player stays in bounds
        if player.y >= (HEIGHT - PLAYER_SIZE) as f64 {
            player.y = (HEIGHT - PLAYER_SIZE) as f64;
            player.velocity_y = 0.0;
            touching_ground = true;
        } else {
            touching_ground = false;
        }

        if player.x >= (WIDTH) as f64 {
            player.x = -(PLAYER_SIZE as f64);
        } else if player.x <= -(PLAYER_SIZE as f64) {
            player.x = (WIDTH) as f64;
        }

        print!("\x1B[1;1H");
        println!("== PLAYER ==");
        println!("Position: {:.2} {:.2}", player.x, player.y);
        println!("Velocity: {:.2} {:.2}", player.velocity_x, player.velocity_y);

        //- DRAW

        // Draw white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw black player
        player.rect.set_x(player.x.round() as i32);
        player.rect.set_y(player.y.round() as i32);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(player.rect).unwrap();

        canvas.present();
        old_ticks = new_ticks;
    }

    Ok(())
}
