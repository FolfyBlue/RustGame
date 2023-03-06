extern crate sdl2;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture}; // cargo build --features "image"
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const GRAVITY: f64 = 0.001;
const MAX_VELOCITY: f64 = 1.0;
const JUMP_VELOCITY: f64 = -0.6;
const MOVE_SPEED: f64 = 0.6;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const PLAYER_WIDTH: u32 = (28.0*2.0) as u32;
const PLAYER_HEIGHT: u32 = (58.0*2.0) as u32;
struct Player<'a> {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    texture: Texture<'a>,
    facing_right: bool,
    rect: Rect,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

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
    let texture_creator = canvas.texture_creator();

    let timer = sdl_context.timer()?;

    // Create a player
    let mut player = Player {
        x: (WIDTH / 2 - PLAYER_WIDTH / 2) as f64,
        y: 0.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        texture: texture_creator.load_texture_bytes(include_bytes!("../assets/player_oc_do_not_steal.png"))?,
        facing_right: true,
        rect: Rect::new(0, 0, PLAYER_WIDTH, PLAYER_HEIGHT),
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
            player.facing_right = false;
        }
        if keys.contains(&Keycode::Right) {
            player.velocity_x = MOVE_SPEED * (delta_time) as f64;
            player.x += MOVE_SPEED * (delta_time) as f64;
            player.facing_right = true;
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
        if player.y >= (HEIGHT - PLAYER_HEIGHT) as f64 {
            player.y = (HEIGHT - PLAYER_HEIGHT) as f64;
            player.velocity_y = 0.0;
            touching_ground = true;
        } else {
            touching_ground = false;
        }

        if player.x >= (WIDTH) as f64 {
            player.x = -(PLAYER_WIDTH as f64);
        } else if player.x <= -(PLAYER_WIDTH as f64) {
            player.x = (WIDTH) as f64;
        }
        
        player.rect.set_x(player.x.round() as i32);
        player.rect.set_y(player.y.round() as i32);

        print!("\x1B[1;1H");
        println!("== PLAYER ==");
        println!("Position: {:.2} {:.2}", player.x, player.y);
        println!("Velocity: {:.2} {:.2}", player.velocity_x, player.velocity_y);

        //- DRAW

        // Draw white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw player
        canvas.copy_ex(
            &player.texture,
            None,
            player.rect,
            0.0,
            None,
            player.facing_right,
            false
        ).unwrap();

        canvas.present();
        old_ticks = new_ticks;
    }

    Ok(())
}
