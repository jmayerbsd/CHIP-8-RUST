const KEYS: usize = 16;

pub struct Keypad {
    pub keys: [bool; KEYS],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false; KEYS],
        }
    }

    pub fn key_press(&mut self, i: u8) {
        self.keys[i as usize] = true;
    }

    pub fn key_release(&mut self, i: u8) {
        self.keys[i as usize] = false;
    }

    pub fn is_key_pressed(&mut self, i: u8) -> bool {
        self.keys[i as usize]
    }
}
