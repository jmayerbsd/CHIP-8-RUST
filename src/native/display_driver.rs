use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::display::Display;
use crate::display::{HEIGHT, WIDTH};

const SCALE_FACTOR: u32 = 20;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &Sdl) -> DisplayDriver {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "rust chip8 demo",
                SCALE_FACTOR * WIDTH as u32,
                SCALE_FACTOR * HEIGHT as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        DisplayDriver { canvas: canvas }
    }

    pub fn draw(&mut self, display: &Display) {
        let pixels = &display.vram;
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(pixels::Color::RGB(
                    col as u8 * 250, // col as u8 * 60,
                    col as u8 * 150, // col as u8 * 60,
                    0,               // col as u8 * 255,
                ));
                let _ = self.canvas.fill_rect(Rect::new(
                    x as i32,
                    y as i32,
                    SCALE_FACTOR,
                    SCALE_FACTOR,
                ));
            }
        }
        self.canvas.present();
    }
}
