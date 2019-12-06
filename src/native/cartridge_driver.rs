use std::fs::File;
use std::io::prelude::*;

use crate::cpu::CPU;
use crate::cpu::ROM_SIZE;

pub struct CartridgeDriver {
    cartridge: [u8; ROM_SIZE],
}

impl CartridgeDriver {
    pub fn new(filename: String) -> CartridgeDriver {
        let mut file = File::open(filename).expect("rom not found");
        let mut cartridge = [0u8; ROM_SIZE];
        file.read(&mut cartridge[..]).expect("failed to read rom");
        CartridgeDriver {
            cartridge: cartridge,
        }
    }

    pub fn load(&mut self, cpu: &mut CPU) {
        cpu.load_cartridge(&self.cartridge);
    }
}
