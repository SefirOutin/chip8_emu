use core::time;
use std::{ops::{BitAnd, BitAndAssign, BitOr, BitXor, Shl, ShlAssign, Shr, ShrAssign}, sync::Mutex};
use macroquad::{color::WHITE, miniquad::native::linux_x11::libx11::Display, prelude::Image};

use macroquad::prelude::{Color, BLACK};


const FONT_SET_LEN: usize = 80;
const FONT_SET: [u8;FONT_SET_LEN] = [
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
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;
pub const SCREEN_SIZE: u16 = SCREEN_HEIGHT as u16 * SCREEN_WIDTH as u16;

pub const TARGET_SCREEN_WIDTH: u16 = 1024;
pub const TARGET_SCREEN_HEIGH: u16 = 512;
pub const SCALE_FACTOR: u16 = 16;

// const CLEAR_SCREEN_SLICE: [Color; TARGET_SCREEN_WIDTH as usize * TARGET_SCREEN_HEIGH as usize] = [WHITE; TARGET_SCREEN_WIDTH as usize * TARGET_SCREEN_HEIGH as usize];

const START_USABLE_RAM: usize = 0x200;
const END_USABLE_RAM: usize = 0xEA0;
const ALL_RAM: usize = 4096;
const AVAIBLE_RAM: usize = END_USABLE_RAM - START_USABLE_RAM;
pub const DISPLAY_BUFFER_START: usize = 0xF00;
const DISPLAY_BUFFER_SIZE: usize = (SCREEN_WIDTH as usize / 8) * (SCREEN_HEIGHT as usize / 8);
const FONT_START_ADDR: usize = 0x50;
const SPRITE_WIDTH: u8 = 8;
const FETCH_STEP: u16 = 2;

const ADDR_MASK: u16 = 0x0FFF;
const LOW_BYTE_MASK: u16 = 0x00FF;
// const HIGH_BYTE_MASK: u16 = 0xFF00;
const N_MASK: u16 = 0x000F;

use std::sync::Arc;

struct Point {
    x: u32,
    y: u32,
}
pub struct Chip8 {
    registers: [u8; 16],
    addr_reg: u16,
    pc: u16,
    stack: [u16; 16],
    stack_ptr: u8,
    d_timer: u8,
    s_timer: u8,
    ram: [u8; ALL_RAM],
    key_inputs: [bool; 16],
    display: (u8, u8),
    virt_display: Arc<Mutex<Image>>,
}

impl Chip8 {
    pub fn new(virt_display_buffer: Arc<Mutex<Image>>) -> Self {
        Self {
            registers: [0; 16],
            addr_reg: 0,
            pc: START_USABLE_RAM as u16,
            stack: [0; 16],
            stack_ptr: 0,
            d_timer: 0,
            s_timer: 0,
            ram: [0; ALL_RAM],
            key_inputs: [false; 16],
            display: (SCREEN_WIDTH, SCREEN_HEIGHT),
            virt_display: virt_display_buffer,
        }
    }


    pub fn init(&mut self, rom: Vec<u8>) {
        self.load_fonts();

        self.load_rom(rom);
    }

    fn load_fonts(&mut self) {
        let font_start_addr = FONT_START_ADDR;

        self.ram[font_start_addr..font_start_addr + FONT_SET_LEN].copy_from_slice(&FONT_SET);
    }

    fn load_rom(&mut self, rom: Vec<u8>) {
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
    
    // #[inline]
    fn draw_square(&self, img: &mut Image, pos: Point, color: Color) {
        println!("setting pixel: x: {0} y: {1} TO x:{2} y:{3}", pos.x, pos.y, pos.x + SCALE_FACTOR as u32, pos.y + SCALE_FACTOR as u32);
        for i in 0..SCALE_FACTOR as u32 {
            for j in 0..SCALE_FACTOR as u32 {
                img.set_pixel(pos.x + j, pos.y + i, color);
            }
        }

    }

    fn project_virt_line_to_display(&self, line_byte: u8, reminder: i16, pos: &Point) {
        use macroquad::prelude::{Color, BLACK, RED, GREEN};
        let mut j: i16 = 0;
        let rm = if reminder < 0 {
            7
        } else {
            reminder
        };

        let mut img = self.virt_display.lock().unwrap();
        
        for i in (0..rm).rev() {
            let display_pos = Point { x: (pos.x * SCALE_FACTOR as u32) as u32, y: pos.y * SCALE_FACTOR as u32 };
            println!("scaled pos: x: {0} y: {1}", display_pos.x, display_pos.y);
            let color: Color;
            if line_byte.shr(i).bitand(1) == 1 {
                color = RED;
            } else {
                color = GREEN;
            }
            self.draw_square(&mut img, display_pos, color);
            j += 1;
        }
        
    }
    
    fn store_bcd_representation(value: u8, dest: &mut [u8]) {
        let factor = 10;
        let mut input = value;

        for i in (0..=2).rev() {
            dest[i] =  input % factor;
            input /= factor;
        }
    }

    pub fn interpret(&mut self) {
        let pause_time = time::Duration::from_millis(5);
        
        loop {
            if self.pc > ALL_RAM as u16 - 1 {
                panic!("invalid prog counter");
            }
            println!("pc: {0}", self.pc);
            let opcode: u16 = (self.ram[self.pc as usize] as u16).shl(8) | self.ram[self.pc as usize + 1] as u16;
            self.pc += FETCH_STEP;
            println!("opcode: 0x{:04X}", opcode);
			match opcode >> 12 {
                0 => {
                    match opcode.bitand(LOW_BYTE_MASK) {
                        0xE0 => {   self.ram[DISPLAY_BUFFER_START..].fill(0);               // CLS
                                    let mut img = self.virt_display.lock().unwrap();

                                    img.update(&[BLACK; TARGET_SCREEN_WIDTH as usize * TARGET_SCREEN_HEIGH as usize]);
                                },

                        0xEE => { self.pc = self.stack[self.stack_ptr as usize]; self.stack_ptr -= 1 },        // RETURN
                        
                        _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
                    }
                } 
                0x1 => { self.pc = opcode.bitand(ADDR_MASK) },     // JP addr
                0x2 => { self.stack_ptr += 1; self.stack[self.stack_ptr as usize] = self.pc; self.pc = opcode.bitand(ADDR_MASK)},        // CALL addr
                0x3 => { if self.get_reg(parse_l_reg(opcode)) == opcode as u8 { self.pc += FETCH_STEP; } },        // SE Rx, byte
                0x4 => { if self.get_reg(parse_l_reg(opcode)) != opcode as u8 { self.pc += FETCH_STEP; } },        // SNE Rx, byte
                0x5 => { if self.get_reg(parse_l_reg(opcode)) == self.get_reg(parse_l_reg(opcode)) { self.pc += FETCH_STEP; } }, // SE Rx, Ry
                0x6 => *self.get_mut_reg(parse_l_reg(opcode)) = opcode.bitand(LOW_BYTE_MASK) as u8,              // LD Rx, byte
                0x7 => *self.get_mut_reg(parse_l_reg(opcode)) += opcode.bitand(LOW_BYTE_MASK) as u8,             // ADD Rx, byte
                0x8 => { match opcode.bitand(N_MASK) {      // match last 4 bits
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
					_ => { println!("Invalid opcode.0x{:04X}", opcode); },
                    
                    }
                }
                0x9 => { if self.get_reg(parse_l_reg(opcode)) != self.get_reg(parse_l_reg(opcode)) { self.pc += FETCH_STEP; } },		// SNE Rx, Ry
                0xA => self.addr_reg = opcode.bitand(ADDR_MASK),															// LD Addr_R, addr (12bits)
                0xB => self.pc = self.get_reg(0) as u16 + opcode.bitand(ADDR_MASK),									// JMP R0, addr (12bits)
                0xC => *self.get_mut_reg(parse_l_reg(opcode)) &= macroquad::rand::gen_range(0, 255),						// RND Rx, byte		
                
                0xD => {																								// DRW Rx, Ry, nibble
					let sprite_len = opcode.bitand(N_MASK) as usize;
					let sprite: Vec<u8> = self.ram[self.addr_reg as usize..(self.addr_reg as usize + sprite_len)].into();
					let mut pos = Point { x: self.get_reg(parse_l_reg(opcode)) as u32, y: self.get_reg(parse_r_reg(opcode)) as u32 };
					
					let reminder: i16 = pos.x as i16 + SPRITE_WIDTH as i16 - SCREEN_WIDTH as i16;
					pos.x = pos.x % (self.display.0 as u32 - 1);
					pos.y = pos.y % (self.display.1 as u32 - 1);
                    
                    self.registers[0xF] = 0;
					for line in sprite {
						if pos.y >= SCREEN_HEIGHT as u32 {
							break;
						}
						
						let ram_pos = DISPLAY_BUFFER_START as u32 + (pos.x / 8) * (pos.y / 8);
                        println!("pos put line chip: {0} {1} | {2} {3}| addr: {4}", pos.x, pos.y, pos.x / 8, pos.y / 8, ram_pos);
						let original_val = self.ram[ram_pos as usize];
						if reminder > 0 {
							self.ram[ram_pos as usize] ^= line.shr(reminder);
						} else {
							self.ram[ram_pos as usize] ^= line;
						}
                        let new_value = self.ram[ram_pos as usize];
						if original_val & !new_value != 0 {     // check if bits have been flipped from 1 to 0
							self.registers[0xF] = 1;
						}

                        self.project_virt_line_to_display(new_value, reminder, &pos);
                        pos.y += 1;
					}
				},
                0xE => match opcode as u8 {
					0x9E => println!("opcode.0x{:04X}", opcode),
					0xA1 => println!("opcode.0x{:04X}", opcode),
					_ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
				}
                0xF => match opcode as u8 {
					0x07 => *self.get_mut_reg(parse_l_reg(opcode)) = self.d_timer,      // LD Rx, DTIMER
					0x0A => println!("opcode.0x{:04X}", opcode),
					0x15 => self.d_timer = *self.get_mut_reg(parse_l_reg(opcode)),      // LD DTIMER, Rx
					0x18 => self.s_timer = *self.get_mut_reg(parse_l_reg(opcode)),      // LD STIMER, Rx
					0x1E => self.addr_reg += self.get_reg(parse_l_reg(opcode)) as u16,  // ADD Addr_R, Rx
					0x29 => println!("opcode.0x{:04X}", opcode),                        // LD 
					0x33 => Self::store_bcd_representation(                             // LD byte, Rx
                        self.get_reg(opcode.bitand(N_MASK) as u8),
                        &mut self.ram[self.addr_reg as usize..self.addr_reg as usize + 2]
                    ),
					0x55 => {                                                           // LD [I], [R]
                        for (i, register) in self.registers.iter().enumerate() {
                            self.ram[self.addr_reg as usize + i] = *register;
                        }
                    },
					0x65 => {                                                           // LD [R], [I]
                        let mut i: usize = 0;
                        for byte in &self.ram[self.addr_reg as usize..self.addr_reg as usize + 15] {
                            self.registers[i] = *byte;
                            i += 1;
                        }
                    },
					_ => { println!("Invalid opcode.0x{:04X}", opcode); return; },

				}
                _ => { println!("Invalid opcode.0x{:04X}", opcode); return; },
            }
            // self.print_state();
			// println!("la");
            std::thread::sleep(pause_time);
        }
    }

	pub fn print_ram(&self) {
        for bytes in self.ram {
            println!("0x{:02X}", bytes);
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
        println!("------------------------------");
        println!("Addr Register Data:");
        for i in 0..15 {
            print!("0x{:02X}   ", self.ram[self.addr_reg as usize + i as usize]);
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