use crate::display::{Display, FONT_CHARS, SPRITE_HEIGHT, SPRITE_WIDTH};
use crate::keypad::Keypad;

use rand::{rngs::ThreadRng, Rng};

const RAM: usize = 4096;
const REGISTERS: usize = 16;
const STACK: usize = 16;
const PROG_START: u16 = 0x200;
const INSTRUCTION_SIZE: u16 = 2;
pub const ROM_SIZE: usize = 3584;

// The chip-8 "machine"
pub struct CPU {
    pub i: u16,  // index register
    pub pc: u16, // program counter
    pub ram: [u8; RAM],
    pub v: [u8; REGISTERS], // registers
    pub keypad: Keypad,     // drivers
    pub display: Display,
    pub stack: [u16; STACK],
    pub sp: u8,         // stack pointer
    pub dt: u8,         // delay timer
    pub st: u8,         // sound timer
    pub rng: ThreadRng, // random number generator
}

impl CPU {
    // inits empty cpu, reset fills these values
    pub fn new() -> CPU {
        CPU {
            i: 0,
            pc: 0,
            ram: [0; RAM],
            v: [0; REGISTERS],
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; STACK],
            sp: 0,
            dt: 0,
            st: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn load_cartridge(&mut self, rom: &[u8]) {
        self.reset();
        self.ram[PROG_START as usize..rom.len() + PROG_START as usize].copy_from_slice(rom);
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = PROG_START;
        self.ram = [0; RAM];
        self.v = [0; REGISTERS];
        self.stack = [0; STACK];
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.ram[..FONT_CHARS.len()].copy_from_slice(&FONT_CHARS);
        self.display.clear();
    }

    fn read_instruction(&mut self) -> u16 {
        (self.ram[self.pc as usize] as u16) << 8 | (self.ram[(self.pc + 1) as usize] as u16)
    }

    pub fn tick(&mut self) {
        let instruction = self.read_instruction();
        self.execute_instruction(instruction);
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1
        }
        if self.st > 0 {
            self.st -= 1
        }
    }

    fn execute_instruction(&mut self, instruction: u16) {
        // break intructions into commonly used slices
        let nibbles = (
            (instruction & 0xF000) >> 12 as u8,
            (instruction & 0x0F00) >> 8 as u8,
            (instruction & 0x00F0) >> 4 as u8,
            (instruction & 0x000F) >> 0 as u8,
        );
        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;

        let (_, x, y, n) = nibbles;

        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        let v0 = self.v[0];

        self.pc += INSTRUCTION_SIZE; // further the pc (can be skipped or jumped later)

        // println!(
        //     "Running instruction {}, {}, {}, {} pc {}",
        //     nibbles.0, nibbles.1, nibbles.2, nibbles.3, self.pc
        // );
        // runs the various operations/instructions
        match nibbles {
            // standard instructions
            // CLS
            (0, 0, 0xE, 0) => self.display.clear(),
            // RET
            (0, 0, 0xE, 0xE) => self.pc = self.stack_pop(),
            // SYS addr (used only on RCA 1802, uncommon, depricated, and unimplemented)
            // (0, _, _, _) => {}
            // JP addr
            (1, _, _, _) => self.jump(nnn),
            // CALL addr
            (2, _, _, _) => {
                self.stack_push(self.pc);
                self.pc = nnn;
            }
            // SE Vx, byte
            (3, _, _, _) => self.pc += (vx == kk as u8) as u16 * INSTRUCTION_SIZE,
            // SNE Vx, byte
            (4, _, _, _) => self.pc += (vx != kk as u8) as u16 * INSTRUCTION_SIZE,
            // SE Vx, Vy
            (5, _, _, 0) => self.pc += (vx == vy as u8) as u16 * INSTRUCTION_SIZE,
            // LD Vx, byte
            (6, _, _, _) => self.v[x as usize] = kk,
            // ADD Vx, byte
            (7, _, _, _) => self.v[x as usize] = (vx as u16 + kk as u16) as u8,
            // LD Vx, Vy
            (8, _, _, 0) => self.v[x as usize] = vy,
            // OR Vx, Vy
            (8, _, _, 1) => self.v[x as usize] |= vy,
            // AND Vx, Vy
            (8, _, _, 2) => self.v[x as usize] &= vy,
            // XOR Vx, Vy
            (8, _, _, 3) => self.v[x as usize] ^= vy,
            // ADD Vx, Vy
            (8, _, _, 4) => {
                let (res, overflow) = vx.overflowing_add(vy);
                self.v[0xf] = overflow as u8;
                self.v[x as usize] = res;
            }
            // SUB Vx, Vy
            (8, _, _, 5) => {
                let (res, borrow) = vx.overflowing_sub(vy);
                self.v[0xf] = !borrow as u8;
                self.v[x as usize] = res;
            }
            // SHR Vx {, Vy}
            (8, _, _, 6) => {
                self.v[0xf] = vx & 1;
                self.v[x as usize] >>= 1;
            }
            // SUBN Vx, Vy
            (8, _, _, 7) => {
                let (res, borrow) = vy.overflowing_sub(vx);
                self.v[0xf] = !borrow as u8;
                self.v[x as usize] = res;
            }
            // ADD Vx {, Vy}
            (8, _, _, 0xE) => {
                self.v[0xf] = vx >> 7;
                self.v[x as usize] <<= 1;
            }
            // SNE Vx, Vy
            (9, _, _, 0) => self.pc += (vx != vy as u8) as u16 * INSTRUCTION_SIZE,
            // LD I, addr
            (0xA, _, _, _) => self.i = nnn,
            // JP V0, addr
            (0xB, _, _, _) => self.pc = nnn + v0 as u16,
            // RND Vx, byte
            (0xC, _, _, _) => self.v[x as usize] = self.rng.gen::<u8>() & kk,
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                self.v[0xf] = self.display.draw(
                    vx as usize,
                    vy as usize,
                    &self.ram[self.i as usize..self.i as usize + n as usize],
                ) as u8
            }
            // SKP Vx
            (0xE, _, 9, 0xE) => self.pc += self.keypad.is_key_pressed(vx) as u16 * INSTRUCTION_SIZE,
            // SKNP Vx
            (0xE, _, 0xA, 1) => {
                self.pc += !self.keypad.is_key_pressed(vx) as u16 * INSTRUCTION_SIZE
            }
            // LD Vx, DT
            (0xF, _, 0, 7) => self.v[x as usize] = self.dt,
            // LD Vx, K
            (0xF, _, 0, 0xA) => {
                self.pc -= INSTRUCTION_SIZE;
                for (i, is_down) in self.keypad.keys.iter().enumerate() {
                    if *is_down {
                        self.v[x as usize] = i as u8;
                        self.pc += INSTRUCTION_SIZE;
                        break;
                    }
                }
            }
            // LD DT, Vx
            (0xF, _, 1, 5) => self.dt = vx,
            // LD ST, Vx
            (0xF, _, 1, 8) => self.st = vx,
            // ADD I, Vx
            (0xF, _, 1, 0xE) => self.i += vx as u16,
            // LD F, Vx
            (0xF, _, 2, 9) => self.i = vx as u16 * SPRITE_HEIGHT as u16 / SPRITE_WIDTH as u16,
            // LD B, Vx
            (0xF, _, 3, 3) => {
                self.ram[self.i as usize + 0] = vx / 100 % 10;
                self.ram[self.i as usize + 1] = vx / 10 % 10;
                self.ram[self.i as usize + 2] = vx / 1 % 10;
            }
            // LD [I], Vx
            (0xF, _, 5, 5) => self.ram[(self.i as usize)..(self.i + x as u16 + 1) as usize]
                .copy_from_slice(&self.v[0..(x as usize + 1)]),
            // LD Vx, [I]
            (0xF, _, 6, 5) => self.v[0..(x as usize + 1)]
                .copy_from_slice(&self.ram[(self.i as usize)..(self.i + x as u16 + 1) as usize]),

            // super chip-48 instuctions
            // (0, 0, 0xC, _) => {}   // SCD nibble
            // (0, 0, 0xF, 0xB) => {} // SCR
            // (0, 0, 0xF, 0xC) => {} // SCL
            // (0, 0, 0xF, 0xD) => {} // EXIT
            // (0, 0, 0xF, 0xE) => {} // LOW
            // (0, 0, 0xF, 0xE) => {} // HIGH
            // (0xD, _, _, 0) => {}   // DRW Vx, Vy, 0
            // (0xF, _, 3, 0) => {}   // LD HF, Vx
            // (0xF, _, 7, 5) => {}   // LD R, Vx
            // (0xF, _, 8, 5) => {}   // LD Vx, R
            // (0, 0, 0, 0) => {}
            (_, _, _, _) => println!(
                "Unsupported instruction {}, {}, {}, {}",
                nibbles.0, nibbles.1, nibbles.2, nibbles.3
            ),
        }
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn stack_push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
