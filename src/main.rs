extern crate sdl2;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture}; // cargo build --features "image"
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const PLAYER_WIDTH: u32 = (28.0 * 2.0) as u32;
const PLAYER_HEIGHT: u32 = (58.0 * 2.0) as u32;

struct Position {
    x: f64,
    y: f64,
}

struct GameObject {
    position: Position,
    rect: Rect,
}

struct TexturedGameObject<'a> {
    game_object: GameObject,
    texture: Texture<'a>,
    flip_horizontal: bool,
    flip_vertical: bool,
    angle: f64,
}
struct Entity<'a> {
    textured_game_object: TexturedGameObject<'a>,
    velocity: Position,
    gravity: f64,
    max_fall_speed: f64,
    is_touching_ground: bool,
}

struct Player<'a> {
    entity: Entity<'a>,
    movement_speed: f64,
    jump_speed: f64,
}

impl Player<'_> {
    fn jump(&mut self, delta_time: i32) {
        if self.entity.is_touching_ground {
            self.entity.velocity.y = -self.jump_speed * (delta_time) as f64;
        }
    }

    fn move_left(&mut self, delta_time: i32) {
        self.entity.velocity.x = -self.movement_speed * (delta_time) as f64;
        self.entity.textured_game_object.game_object.position.x += -self.movement_speed * (delta_time) as f64;
        self.entity.textured_game_object.flip_horizontal = false;
    }

    fn move_right(&mut self, delta_time: i32) {
        self.entity.velocity.x = self.movement_speed * (delta_time) as f64;
        self.entity.textured_game_object.game_object.position.x += self.movement_speed * (delta_time) as f64;
        self.entity.textured_game_object.flip_horizontal = true;
    }

    fn update(&mut self, delta_time: i32, keys: HashSet<Keycode>) {
        // Handle movement
        if keys.contains(&Keycode::Left) {
            self.move_left(delta_time);
        }
        if keys.contains(&Keycode::Right) {
            self.move_right(delta_time);
        }
        if keys.contains(&Keycode::Up) {
            self.jump(delta_time);
        }

        // Ensure the player doesn't surpass the maximum fall velocity
        if self.entity.velocity.y <= self.entity.max_fall_speed {
            self.entity.velocity.y += self.entity.gravity * (delta_time) as f64;
        } else {
            self.entity.velocity.y = self.entity.max_fall_speed;
        }
        self.entity.velocity.x = self.entity.velocity.x * (delta_time) as f64;

        // Update the player position according to the given velocity
        self.entity.textured_game_object.game_object.position.x += self.entity.velocity.x * (delta_time as f64);
        self.entity.textured_game_object.game_object.position.y += self.entity.velocity.y * (delta_time as f64);

        // Ensure the player stays in bounds
        if self.entity.textured_game_object.game_object.position.y >= (HEIGHT - PLAYER_HEIGHT) as f64
        {
            self.entity.textured_game_object.game_object.position.y = (HEIGHT - PLAYER_HEIGHT) as f64;
            self.entity.velocity.y = 0.0;
            self.entity.is_touching_ground = true;
        } else {
            self.entity.is_touching_ground = false;
        }

        if self.entity.textured_game_object.game_object.position.x >= (WIDTH) as f64 {
            self.entity.textured_game_object.game_object.position.x = -(PLAYER_WIDTH as f64);
        } else if self.entity.textured_game_object.game_object.position.x <= -(PLAYER_WIDTH as f64)
        {
            self.entity.textured_game_object.game_object.position.x = (WIDTH) as f64;
        }

        self.entity.textured_game_object.game_object.rect.set_x(
            self.entity
                .textured_game_object
                .game_object
                .position
                .x
                .round() as i32,
        );
        self.entity.textured_game_object.game_object.rect.set_y(
            self.entity
                .textured_game_object
                .game_object
                .position
                .y
                .round() as i32,
        );
    }
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

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let timer = sdl_context.timer()?;

    // Create a player

    let mut player = Player {
        entity: Entity {
            textured_game_object: TexturedGameObject {
                game_object: GameObject {
                    position: Position {
                        x: (WIDTH / 2 - PLAYER_WIDTH / 2) as f64,
                        y: 0.0,
                    },
                    rect: Rect::new(0, 0, PLAYER_WIDTH, PLAYER_HEIGHT),
                },
                texture: texture_creator
                    .load_texture_bytes(include_bytes!("../assets/player_oc_do_not_steal.png"))?,
                flip_horizontal: false,
                flip_vertical: false,
                angle: 0.0,
            },
            velocity: Position { x: 0.0, y: 0.0 },
            gravity: 0.001,
            max_fall_speed: 1.0,
            is_touching_ground: false,
        },
        movement_speed: 0.6,
        jump_speed: 0.6,
    };

    let mut event_pump = sdl_context.event_pump()?;

    let mut old_ticks: i32 = 0;

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
        player.update(delta_time, keys);

        print!("\x1B[1;1H");
        println!("== PLAYER ==");
        println!(
            "Position: {:.2} {:.2}",
            player.entity.textured_game_object.game_object.position.x,
            player.entity.textured_game_object.game_object.position.y
        );
        println!(
            "Velocity: {:.2} {:.2}",
            player.entity.velocity.x, player.entity.velocity.y
        );

        //- DRAW

        // Draw white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw player
        canvas
            .copy_ex(
                &player.entity.textured_game_object.texture,
                None,
                player.entity.textured_game_object.game_object.rect,
                player.entity.textured_game_object.angle,
                None,
                player.entity.textured_game_object.flip_horizontal,
                player.entity.textured_game_object.flip_vertical,
            )
            .unwrap();

        canvas.present();
        old_ticks = new_ticks;
    }

    Ok(())
}
