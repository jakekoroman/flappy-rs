use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

const FPS: u32 = 30;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const PLAYER_VELOCITY: i32 = 50;
const GRAVITY: i32 = 5;

const PIPE_COLOR: Color = Color::RGB(11, 224, 68);
const PIPE_X_POS: i32 = 325;
const PIPE_GAP: i32 = 100;
const PIPE_WIDTH: u32 = 75;
const PIPE_SPEED: i32 = 5;

#[derive(Copy, Clone)]
struct Vector2<T: Add + Sub + Div + Mul> {
    x: T,
    y: T,
}

impl<T: Add<Output = T> + Sub + Div + Mul> Add<Vector2<T>> for Vector2<T> {
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Player {
    pos: Rect,
    color: Color,
}

impl Player {
    fn new(pos: Rect, color: Color) -> Self {
        Player { pos, color }
    }
}

#[derive(Debug)]
struct Pipe {
    top_pos: Rect,
    bot_pos: Rect,
}

impl Pipe {
    fn new(top_pos: Rect, bot_pos: Rect) -> Self {
        Pipe { top_pos, bot_pos }
    }
}

fn draw<T: RenderTarget>(canvas: &mut Canvas<T>, player: &Player, pipes: &VecDeque<Pipe>) {
    canvas.set_draw_color(Color::RGB(100, 157, 250));
    canvas.clear();

    canvas.set_draw_color(player.color);
    canvas.fill_rect(player.pos).unwrap();

    canvas.set_draw_color(PIPE_COLOR);
    for pipe in pipes.iter() {
        canvas.fill_rect(pipe.top_pos).unwrap();
        canvas.fill_rect(pipe.bot_pos).unwrap();
    }

    canvas.present();
}

fn check_loss(player: &Player, pipes: &VecDeque<Pipe>) -> bool {
    let pipe = pipes.get(0).unwrap();

    if player.pos.x >= pipe.top_pos.x && player.pos.x <= pipe.top_pos.x + PIPE_WIDTH as i32 {
        if !(player.pos.y > pipe.top_pos.height() as i32
            && player.pos.y < (pipe.top_pos.height() + PIPE_GAP as u32) as i32)
        {
            println!("----");
            dbg!(player.pos.x);
            dbg!(player.pos.y);
            dbg!(pipe.top_pos.x);
            dbg!(pipe.top_pos.height());
            dbg!(pipe.top_pos.height() + PIPE_GAP as u32);
            println!("----");
            return false;
        }
    }

    true
}

fn update(player: &mut Player, pipes: &mut VecDeque<Pipe>, frame_count: i32) {
    if !(player.pos.y + player.pos.h >= WINDOW_HEIGHT) {
        player.pos.y += GRAVITY;
    }

    let mut rng = thread_rng();
    if frame_count % (FPS * 4) as i32 == 0 {
        let mut n: i32 = rng.gen_range(100..WINDOW_HEIGHT - 100);

        let mut top_pos = Rect::new(PIPE_X_POS, 0, PIPE_WIDTH, n as u32);
        let mut bot_pos = Rect::new(
            PIPE_X_POS,
            n + PIPE_GAP,
            PIPE_WIDTH,
            (WINDOW_HEIGHT - PIPE_GAP) as u32,
        );
        pipes.push_back(Pipe::new(top_pos, bot_pos));

        n = rng.gen_range(100..WINDOW_HEIGHT - 100);
        top_pos = Rect::new(PIPE_X_POS * 2, 0, PIPE_WIDTH, n as u32);
        bot_pos = Rect::new(
            PIPE_X_POS * 2,
            n + PIPE_GAP,
            PIPE_WIDTH,
            (WINDOW_HEIGHT - PIPE_GAP) as u32,
        );
        pipes.push_back(Pipe::new(top_pos, bot_pos));
    }

    if pipes.get(0).unwrap().top_pos.x <= -(PIPE_WIDTH as i32) {
        pipes.pop_front();
    }

    for pipe in pipes.iter_mut() {
        pipe.top_pos.x -= PIPE_SPEED;
        pipe.bot_pos.x -= PIPE_SPEED;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Flappy Bird", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FPS).unwrap();

    let player_loc = Vector2 { x: 35, y: 100 };
    let player_size = Vector2 { x: 30, y: 30 };
    let mut player = Player::new(
        Rect::new(player_loc.x, player_loc.y, player_size.x, player_size.y),
        Color::RGB(252, 242, 101),
    );

    let mut pipes: VecDeque<Pipe> = VecDeque::new();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    player.pos.y -= PLAYER_VELOCITY;
                }

                _ => {}
            }
        }

        update(&mut player, &mut pipes, fps_manager.get_frame_count());

        if !check_loss(&player, &pipes) {
            break 'main_loop;
        }

        draw(&mut canvas, &player, &pipes);
        fps_manager.delay();
    }

    Ok(())
}
