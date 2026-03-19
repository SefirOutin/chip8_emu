use std::ops::{BitAnd, BitAndAssign, BitOr, BitXor, Shl, ShlAssign, Shr, ShrAssign};
use macroquad::{color::BLACK, window::clear_background, *};
use rand;

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
const DISPLAY_BUFFER_START: usize = 0xF00 / 2;
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
    stack: [u16; 16],
    // stack: stack,
    stack_ptr: u8,
    d_timer: u8,
    s_timer: u8,
    ram: [u16; ALL_RAM],
    key_inputs: [u8; 16],
    display: (u8, u8),
    
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            addr_reg: 0,
            pc: START_USABLE_RAM as u16,
            stack: [0; 16],
            // stack: stack { stack: [0; 16] },
            stack_ptr: 0,
            d_timer: 0,
            s_timer: 0,
            ram: [0; ALL_RAM],
            key_inputs: [0; 16],
            display: (SCREEN_WIDTH, SCREEN_HEIGHT),
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

    fn get_reg(&self, x: u8) -> u8{

        self.registers[x as usize]
    }

    
    fn get_mut_reg(&mut self, x: u8) -> &mut u8 {
        &mut self.registers[x as usize]
    }

    pub fn interpret(&mut self) {
        loop {
            if self.pc > ALL_RAM as u16 - 1 {
                self.pc = ALL_RAM as u16 - 1;
            }
            let opcode = self.ram[self.pc as usize];
            match opcode >> 12 {
                0 => {
                    match opcode.bitand(0x00FF) {
                        0xE0 => { clear_background(BLACK);              // CLS
                            self.ram[DISPLAY_BUFFER_START as usize..ALL_RAM as usize - 1].fill(0); 
                        }

                        0xEE => { self.pc = self.stack[self.stack_ptr as usize]; self.stack_ptr -= 1; },        // RETURN
                        
                        _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
                    }
                } 
                0x1 => { self.pc = opcode.bitand(0x0FFF) / 2},     // JP addr
                0x2 => { self.stack_ptr += 1; self.stack[self.stack_ptr as usize] = self.pc; self.pc = opcode.bitand(0x0FFF) / 2; },        // CALL addr
                0x3 => { if self.get_reg(parse_l_reg(opcode)) == opcode as u8 { self.pc += 1; } },        // SE Rx, byte
                0x4 => { if self.get_reg(parse_l_reg(opcode)) != opcode as u8 { self.pc += 1; } },        // SNE Rx, byte
                0x5 => { if self.get_reg(parse_l_reg(opcode)) == self.get_reg(parse_l_reg(opcode)) { self.pc += 1; } }, // SE Rx, Ry
                0x6 => *self.get_mut_reg(parse_l_reg(opcode)) = opcode.bitand(0x00FF) as u8,              // LD Rx, byte
                0x7 => *self.get_mut_reg(parse_l_reg(opcode)) += opcode.bitand(0x00FF) as u8,             // ADD Rx, byte
                0x8 => { match opcode.bitand(0x000F) {      // match last 4 bits
                    0x0 => *self.get_mut_reg(parse_l_reg(opcode)) = self.get_reg(parse_r_reg(opcode)),                                      // LD Rx, Ry
                    0x1 => *self.get_mut_reg(parse_l_reg(opcode)) |= self.get_reg(parse_r_reg(opcode)),    // OR Rx, Ry
                    0x2 => *self.get_mut_reg(parse_l_reg(opcode)) &= self.get_reg(parse_r_reg(opcode)),     // AND Rx, Ry
                    0x3 => *self.get_mut_reg(parse_l_reg(opcode)) ^= self.get_reg(parse_r_reg(opcode)),    // XOR Rx, Ry
                    0x4 => {                                                                                                            // ADD Rx, Ry
                        let out: u16 = (self.get_reg(parse_l_reg(opcode)) + self.get_reg(parse_r_reg(opcode))) as u16;
                        *self.get_mut_reg(parse_l_reg(opcode)) = out as u8;
                        if out > u8::MAX as u16 {
                            *self.get_mut_reg(0xF) = 1;
                        } else {
                            *self.get_mut_reg(0xF) = 0;
                        }
                    },
                    0x5 => {                                                                                                            // SUB Rx, Ry
						if self.get_reg(parse_l_reg(opcode)) > self.get_reg(parse_l_reg(opcode)) {
							*self.get_mut_reg(0xF) = 1;
						} else {
							*self.get_mut_reg(0xF) = 0;
						}
						*self.get_mut_reg(parse_l_reg(opcode)) -= self.get_reg(parse_r_reg(opcode));
                    },
                    0x6 => {                                                                                                            // SHR Rx, Ry
						if self.get_reg(parse_l_reg(opcode)).bitand(1) == 1 {		// least-significant bit
							*self.get_mut_reg(0xF) = 1;
						} else {
							*self.get_mut_reg(0xF) = 0;
						}
						(*self.get_mut_reg(parse_l_reg(opcode))).shr_assign(1);
                    },
                    0x7 => {                                                                                                            // SUBN Rx, Ry
						if self.get_reg(parse_l_reg(opcode)) < self.get_reg(parse_l_reg(opcode)) {
							*self.get_mut_reg(0xF) = 1;
						} else {
							*self.get_mut_reg(0xF) = 0;
						}
						*self.get_mut_reg(parse_r_reg(opcode)) -= self.get_reg(parse_l_reg(opcode));
                    },
                    0xE => {                                                                                                            // SHL Rx, Ry
						if self.get_reg(parse_l_reg(opcode)).bitand(0x80) == 1 {		// most-significant bit
							*self.get_mut_reg(0xF) = 1;
						} else {
							*self.get_mut_reg(0xF) = 0;
						}
						(*self.get_mut_reg(parse_l_reg(opcode))).shl_assign(1);
                    },
					_ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
                    
                    }
                }
                0x9 => { if self.get_reg(parse_l_reg(opcode)) != self.get_reg(parse_l_reg(opcode)) { self.pc += 1; } },		// SNE Rx, Ry
                0xA => self.addr_reg = opcode.bitand(0x0FFF),															// LD Addr_R, addr (12bits)
                0xB => self.pc = (self.get_reg(0) as u16 + opcode.bitand(0x0FFF)) / 2,									// JMP R0, addr (12bits)
                0xC => *self.get_mut_reg(parse_l_reg(opcode)) &= rand::gen_range(0, 255),						// RND Rx, byte		
                0xD => {																								// DRW Rx, Ry, nibble
					let sprite_len = opcode.bitand(0x000F) as usize;
					let sprite = &self.ram[self.addr_reg as usize..(self.addr_reg as usize + sprite_len)];
					let (mut x, mut y) = (self.get_reg(parse_l_reg(opcode)), self.get_reg(parse_r_reg(opcode)));

					// let pos = (self.get_reg(parse_l_reg(opcode)) * self.get_reg(parse_r_reg(opcode))) as usize;
					for (i, pixels) in sprite.iter().enumerate() {		// 2 pixels
						x += i as u8;
						if x > SCREEN_WIDTH {
							x = 0;
						}
					}
					// for bytes in self.ram[pos..pos + sprite_len].into_iter() {
					// 	bytes ^= sprite[i];
					// }
				},
                0xE => match opcode as u8 {
					0x9E => ,
					0xA1 => ,
					_ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
				}
                0xF => match opcode as u8 {
					0x07 => ,
					0x0A => ,
					0x15 => ,
					0x18 => ,
					0x1E => ,
					0x29 => ,
					0x33 => ,
					0x55 => ,
					0x65 => ,
					_ => { println!("Invalid opcode.0x{:04X}", opcode); return; },

				}
                _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
            }
            self.print_state();
            self.pc += 1;
            println!("opcode: 0x{:02X}", opcode);
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
fn parse_l_reg(opcode: u16) -> u8 {
    (opcode.shr(8) as u8).bitand(0x0F)
}
#[inline]
fn parse_r_reg(opcode: u16) -> u8 {
    (opcode.shr(4) as u8).bitand(0x0F)
}
#[inline]
fn parse_both_reg(opcode: u16) -> (u8, u8) {
    (parse_l_reg(opcode), parse_r_reg(opcode))
}