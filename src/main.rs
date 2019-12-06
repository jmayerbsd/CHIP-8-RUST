extern crate rand;
extern crate sdl2;

use std::env;
use std::thread;
use std::time::Duration;

use sdl2::audio::AudioStatus;

pub mod cpu;
pub mod display;
pub mod keypad;
pub mod native;

use cpu::CPU;

use native::CartridgeDriver;
use native::DisplayDriver;
use native::InputDriver;
use native::SoundDriver;

fn main() {
    println!("Running CHIP8 emulator");

    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "specify rom");
    let filename = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let mut cpu = CPU::new();
    let mut cartridge_driver = CartridgeDriver::new(filename.to_string());
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut sound_driver = SoundDriver::new(&sdl_context);

    cartridge_driver.load(&mut cpu);

    loop {
        display_driver.draw(&cpu.display);
        if input_driver.update(&mut cpu.keypad) {
            break;
        }

        cpu.tick_timers();

        for _ in 0..8 {
            cpu.tick();
        }

        if cpu.st > 0 && sound_driver.device.status() != AudioStatus::Playing {
            sound_driver.begin_beep();
        }
        if cpu.st == 0 && sound_driver.device.status() == AudioStatus::Playing {
            sound_driver.end_beep();
        }

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
