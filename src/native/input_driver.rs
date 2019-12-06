use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::Sdl;

use std::collections::HashMap;

use crate::keypad::Keypad;

pub struct InputDriver {
    event_pump: EventPump,
    keycodes: HashMap<Keycode, u8>,
}

impl InputDriver {
    pub fn new(sdl_context: &Sdl) -> InputDriver {
        let event_pump = sdl_context.event_pump().unwrap();
        let keycodes = [
            (Keycode::Num1, 0x1),
            (Keycode::Num2, 0x2),
            (Keycode::Num3, 0x3),
            (Keycode::Num4, 0xc),
            (Keycode::Q, 0x4),
            (Keycode::W, 0x5),
            (Keycode::E, 0x6),
            (Keycode::R, 0xd),
            (Keycode::A, 0x7),
            (Keycode::S, 0x8),
            (Keycode::D, 0x9),
            (Keycode::F, 0xe),
            (Keycode::Z, 0xa),
            (Keycode::X, 0x0),
            (Keycode::C, 0xb),
            (Keycode::V, 0xf),
        ]
        .iter()
        .cloned()
        .collect();
        InputDriver {
            event_pump: event_pump,
            keycodes: keycodes,
        }
    }

    pub fn update(&mut self, keypad: &mut Keypad) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if self.keycodes.contains_key(&keycode) => {
                    keypad.key_press(*self.keycodes.get(&keycode).unwrap())
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } if self.keycodes.contains_key(&keycode) => {
                    keypad.key_release(*self.keycodes.get(&keycode).unwrap())
                }
                _ => {}
            }
        }
        false
    }
}
