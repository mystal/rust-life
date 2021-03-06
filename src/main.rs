#![feature(conservative_impl_trait)]
#![feature(test)]

extern crate rand;
extern crate sdl2;

extern crate test;


use sdl2::{
    EventPump,
    TimerSubsystem,
    VideoSubsystem,
};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{
    //BlendBlend,
    Renderer,
};

mod board;


const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 500;

const BOARD_WIDTH: u64 = 50;
const BOARD_HEIGHT: u64 = 50;

const FPS: u32 = 60;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);


fn draw_board(renderer: &mut Renderer, board: &board::LifeBoard) {
    let cell_width = SCREEN_WIDTH / BOARD_WIDTH as u32;
    let cell_height = SCREEN_HEIGHT / BOARD_HEIGHT as u32;

    renderer.set_draw_color(WHITE);
    for cell in board.iter_live_cells() {
        let cell_rect = Rect::new(
            (cell.x * cell_width as i64) as i32,
            (cell.y * cell_height as i64) as i32,
            cell_width,
            cell_height,
        );
        renderer.fill_rect(cell_rect);
    }
}

fn run(video: VideoSubsystem, mut timer: TimerSubsystem, mut event_pump: EventPump,
       step_time_ms: u32) {
    let mut board = board::LifeBoard::new();

    let window = video.window(
        "rust-life",
        SCREEN_WIDTH,
        SCREEN_HEIGHT)
        .build().unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();
    //renderer.set_blend_mode(BlendBlend);

    let mut running = true;
    let mut simulate = false;
    let mut last_step_time = timer.ticks();

    let cell_width = SCREEN_WIDTH as u64 / BOARD_WIDTH;
    let cell_height = SCREEN_HEIGHT as u64 / BOARD_HEIGHT;

    while running {
        let start_time = timer.ticks();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::mouse::Mouse;

            match event {
                Event::KeyDown {keycode: Some(key), ..} => {
                    println!("{:?}", key);
                    match key {
                        Keycode::Space => simulate = !simulate,
                        //Keycode::R => board.randomize(),
                        Keycode::C => board.clear(),
                        Keycode::S => board.step(),
                        Keycode::Escape => running = false,
                        _ => {},
                    }
                },
                Event::MouseButtonDown {mouse_btn: Mouse::Left, x, y, ..} => {
                    board.set((x as u64 / cell_width) as i64,
                              (y as u64 / cell_height) as i64,
                              true);
                },
                Event::MouseButtonDown {mouse_btn: Mouse::Right, x, y, ..} => {
                    board.set((x as u64 / cell_width) as i64,
                              (y as u64 / cell_height) as i64,
                              false);
                },
                Event::Quit {..} => running = false,
                _ => {},
            }
        }

        let current_time = timer.ticks();
        if simulate && (current_time - last_step_time >= step_time_ms) {
            board.step();
            last_step_time = timer.ticks();
        }

        // Clear the screen
        renderer.set_draw_color(BLACK);
        renderer.clear();

        // Draw
        draw_board(&mut renderer, &board);

        // Update buffers
        renderer.present();

        let ms_per_frame = 1000 / FPS;
        let elapsed_time = timer.ticks() - start_time;
        if elapsed_time < ms_per_frame {
            timer.delay(ms_per_frame - elapsed_time);
        }
    }
}

fn main() {
    let step_time = 200;

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let timer = sdl.timer().unwrap();
    let event_pump = sdl.event_pump().unwrap();

    run(video, timer, event_pump, step_time);
}
