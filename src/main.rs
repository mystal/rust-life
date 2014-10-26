extern crate sdl2;

use sdl2::event;
use sdl2::keycode;
use sdl2::pixels;
use sdl2::render::{
    ACCELERATED,
    //BlendBlend,
    DriverAuto,
    Renderer,
};
use sdl2::timer;
use sdl2::video::{
    PosUndefined,
    SHOWN,
    Window,
};

mod grid;

const SCREEN_WIDTH: uint = 500;
const SCREEN_HEIGHT: uint = 500;

const FPS: uint = 60;

const BLACK: pixels::Color = pixels::RGB(0, 0, 0);
const WHITE: pixels::Color = pixels::RGB(255, 255, 255);

fn draw_board(renderer: &Renderer<Window>, board: &grid::LifeBoard) {
    let mut cell_rect = sdl2::rect::Rect {
        x: 0,
        y: 0,
        w: (SCREEN_WIDTH/board.grid.width) as i32,
        h: (SCREEN_HEIGHT/board.grid.height) as i32,
    };

    renderer.set_draw_color(WHITE);
    for (j, bv) in board.grid.grid.iter().enumerate() {
        for (i, alive) in bv.iter().enumerate() {
            if alive {
                cell_rect.x = i as i32 * cell_rect.w;
                cell_rect.y = j as i32 * cell_rect.h;

                renderer.fill_rect(&cell_rect);
            }
        }
    }
}

fn run(step_time_ms: uint, width: uint, height: uint) {
    let mut board = grid::LifeBoard::new(width, height);

    let window = Window::new("Life",
                             PosUndefined,
                             PosUndefined,
                             SCREEN_WIDTH as int,
                             SCREEN_HEIGHT as int,
                             SHOWN).unwrap();
    let renderer = Renderer::from_window(
        window, DriverAuto, ACCELERATED).unwrap();
    //renderer.set_blend_mode(BlendBlend);

    sdl2::mouse::show_cursor(true);

    let mut running = true;
    let mut simulate = false;
    let mut last_step_time = timer::get_ticks();

    let cell_width = SCREEN_WIDTH/board.grid.width;
    let cell_height = SCREEN_HEIGHT/board.grid.height;

    while running {
        let start_time = timer::get_ticks();

        loop {
            match event::poll_event() {
                event::KeyDownEvent(_, _, key, _, _) => {
                    println!("{}", key);
                    match key {
                        keycode::SpaceKey => simulate = !simulate,
                        keycode::RKey => board.randomize(),
                        keycode::CKey => board.clear(),
                        keycode::SKey => board.step(),
                        _ => continue,
                    }
                },
                event::MouseButtonDownEvent(_, _, _, sdl2::mouse::LeftMouse, x, y) => {
                    board.grid.set(x as uint/cell_width, y as uint/cell_height, true);
                },
                event::MouseButtonDownEvent(_, _, _, sdl2::mouse::RightMouse, x, y) => {
                    board.grid.set(x as uint/cell_width, y as uint/cell_height, false);
                },
                event::QuitEvent(..) => running = false,
                event::NoEvent => break,
                _ => continue,
            }
        }

        let current_time = timer::get_ticks();
        if simulate && (current_time - last_step_time >= step_time_ms) {
            board.step();
            last_step_time = timer::get_ticks();
        }

        // Clear the screen
        renderer.set_draw_color(BLACK);
        renderer.clear();

        // Draw
        draw_board(&renderer, &board);

        // Update buffers
        renderer.present();

        let ms_per_frame = 1000 / FPS;
        let elapsed_time = timer::get_ticks() - start_time;
        if elapsed_time < ms_per_frame {
            timer::delay(ms_per_frame - elapsed_time);
        }
    }
}

fn main() {
    let step_time = 200;
    let width = 50;
    let height = 50;

    sdl2::init(sdl2::INIT_VIDEO | sdl2::INIT_TIMER);

    run(step_time, width, height);

    sdl2::quit();
}
