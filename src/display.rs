// use std::mem::swap;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SPRITE_WIDTH: usize = 8;
pub const SPRITE_HEIGHT: usize = 5 * 8;

pub struct Display {
    pub vram: [[u8; WIDTH]; HEIGHT],
    // pub front_buffer: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            vram: [[0; WIDTH]; HEIGHT],
            // front_buffer: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool) {
        self.vram[y][x] = state as u8;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.vram[y][x] == 1
    }

    pub fn clear(&mut self) {
        // println!("swapped");
        // swap(&mut self.vram, &mut self.front_buffer);
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                self.vram[row][col] = 0;
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collides = false;
        for (j, row) in sprite.iter().enumerate() {
            for i in 0..SPRITE_WIDTH {
                let value = row >> (7 - i) & 1 == 1;

                let x1 = (x + i) % WIDTH;
                let y1 = (y + j) % HEIGHT;

                let old = self.get_pixel(x1, y1);
                collides |= old && value;

                self.set_pixel(x1, y1, value ^ old);
            }
        }
        collides
    }
}

pub const FONT_CHARS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
