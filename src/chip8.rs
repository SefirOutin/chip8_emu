use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use macroquad::{color::BLACK, window::clear_background, *};

const FONT_SET_LEN: usize = 48;
const FONT_SET: [u16;FONT_SET_LEN] = [
    0xF090, 0x9090, 0x00F0, // 0
	0x2060, 0x2020, 0x0070, // 1
	0xF010, 0xF080, 0x00F0, // 2
	0xF010, 0xF010, 0x00F0, // 3
	0x9090, 0xF010, 0x0010, // 4
	0xF080, 0xF010, 0x00F0, // 5
	0xF080, 0xF090, 0x00F0, // 6
	0xF010, 0x2040, 0x0040, // 7
	0xF090, 0xF090, 0x00F0, // 8
	0xF090, 0xF010, 0x00F0, // 9
	0xF090, 0xF090, 0x0090, // A
	0xE090, 0xE090, 0x00E0, // B
	0xF080, 0x8080, 0x00F0, // C
	0xE090, 0x9090, 0x00E0, // D
	0xF080, 0xF080, 0x00F0, // E
	0xF080, 0xF080, 0x0080  // F
];

const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 32;
// [u8] stored in Vec<u16> so: / 2
const START_USABLE_RAM: usize = 0x200 / 2;
const END_USABLE_RAM: usize = 0xEA0 / 2;
const ALL_RAM: usize = 4096 / 2;
const AVAIBLE_RAM: usize = END_USABLE_RAM - START_USABLE_RAM;
const DISPLAY_BUFFER: usize = 0xF00 / 2;
const FONT_START_ADDR: usize = 0x50 / 2;

struct Display {
    width: u8,
    height: u8,
}

impl Display {
    fn new(width: u8, height: u8) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}

pub struct Chip8 {
    registers: [u8; 16],
    addr_reg: u16,
    pc: u16,
    stack_ptr: u8,
    d_timer: u8,
    s_timer: u8,
    ram: [u16; ALL_RAM],
    key_inputs: [u8; 16],
    display: Display,
    
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            addr_reg: 0,
            pc: START_USABLE_RAM as u16,
            // STACK LEVEL ???
            stack_ptr: 0,
            d_timer: 0,
            s_timer: 0,
            ram: [0; ALL_RAM],
            key_inputs: [0; 16],
            display: Display { width: SCREEN_WIDTH, height: SCREEN_HEIGHT },
        }
    }

    pub fn load_fonts(&mut self) {
        let font_start_addr = FONT_START_ADDR;

        self.ram[font_start_addr..font_start_addr + FONT_SET_LEN].copy_from_slice(&FONT_SET);
    }

    pub fn load_rom(&mut self, rom: Vec<u16>) {
        let len_bin = rom.len();
        if len_bin > AVAIBLE_RAM {
            panic!("File too large...");
        }

        self.ram[self.pc as usize..self.pc as usize + len_bin ].clone_from_slice(&rom);
    }

    fn registers_manip(&self, opcode: u16) {
        println!("reg manip opcode: 0x{:04X}", opcode);
    //     let reg_a = (opcode.shl(8) as u8).bitxor(0x0F);
    //     let reg_b: Option<u8> = match opcode >> 12 {
    //         5 | 8 | 9 => Some(opcode.shl(4) as u8),
    //         _ => None,
    //     };
    }

    pub fn interpret(&mut self) {
        loop {
        // for opcode in self.ram {
            // let op: u16;
            // if opcode.len() == 2 {
            //     op = ((opcode[0] as u16) << 8).bitand(opcode[1] as u16);
            // } else {
            //     return;
            // }
            if self.pc > ALL_RAM as u16 - 1 {
                self.pc = ALL_RAM as u16 - 1;
            }
            let opcode = self.ram[self.pc as usize];
            match opcode >> 12 {
                0 => {
                    match opcode.bitand(0x00FF) {
                        0xE0 => { clear_background(BLACK); 
                        // self.ram[DISPLAY_BUFFER as usize..ALL_RAM as usize - 1].fill(0); 
                        }
                        0xEE => println!("0x00EE"),
                        // 0xEE => { self.pc = }        TODO stack call
                        _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
                    }
                } 
                0x1 => { self.pc = opcode.bitand(0x0FFF) / 2},     // 1XXX
                0x2 => { self.pc = opcode.bitand(0x0FFF) / 2; /*  TODO stack handle */ },
                0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 | 0xC | 0xD | 0xE | 0xF => self.registers_manip(opcode),
                0xA => self.addr_reg = opcode.bitand(0x0FFF),
                0xB => self.pc = (self.registers[0] as u16 + opcode.bitand(0x0FFF)) / 2,
                // 0xC => ,
                // 0xD => ,
                // 0xE => ,
                // 0xF => ,
                _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
            }
            self.print_state();
            self.pc += 1;
        }
    }

	pub fn print_ram(&self) {
        for bytes in self.ram {
            println!("0x{:02X} 0x{:02X}", bytes >> 8, bytes.bitand(0x00FF));
        }
    }

    pub fn print_state(&self) {
        println!("========== VM STATE ==========");
        
        // 1. Core Pointers & Indexes (Hexadecimal)
        println!(
            "PC: 0x{:04X} | I (Addr): 0x{:04X} | SP: 0x{:02X}", 
            self.pc, self.addr_reg, self.stack_ptr
        );
        
        // 2. Timers (Decimal, as they just count down from 60Hz)
        println!("DT (Delay): {:03}  | ST (Sound): {:03}", self.d_timer, self.s_timer);
        
        println!("------------------------------");
        println!("Registers (V0 - VF):");
        
        // 3. Registers printed in two neat rows of 8
        for i in 0..8 {
            print!("V{:X}: 0x{:02X}   ", i, self.registers[i]);
        }
        println!(); // Newline between rows
        for i in 8..16 {
            print!("V{:X}: 0x{:02X}   ", i, self.registers[i]);
        }
        
        println!("\n==============================\n");
    }

}


#[inline]
fn get_l_reg(opcode: u16) -> u8 {
    (opcode.shr(8) as u8).bitand(0x0F)
}
#[inline]
fn get_r_reg(opcode: u16) -> u8 {
    opcode.shr(4) as u8
}
#[inline]
fn get_both_reg(opcode: u16) -> (u8, u8) {
    (get_l_reg(opcode), get_r_reg(opcode))
}