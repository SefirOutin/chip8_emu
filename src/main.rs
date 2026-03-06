use std::fs;
use std::env;


const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 32;
const START_USABLE_RAM: u16 = 0x200;
const ALL_RAM: u16 = 4096;
const MAX_RAM: u16 = ALL_RAM - START_USABLE_RAM;

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

struct Chip8 {
    registers: [u8; 16],
    index_reg: u16,
    pc: u16,
    stack_ptr: u8,
    d_timer: u8,
    s_timer: u8,
    ram: [u8; 4096],
    key_inputs: [u8; 16],
    display: Display,
    
}

impl Chip8 {
    fn new() -> Self {
        Self {
            registers: [0; 16],
            index_reg: 0,
            pc: START_USABLE_RAM,
            stack_ptr: 0,
            d_timer: 0,
            s_timer: 0,
            ram: [0; 4096],
            key_inputs: [0; 16],
            display: Display { width: SCREEN_WIDTH, height: SCREEN_HEIGHT },
        }
    }

    pub fn load_rom(&mut self) {

        if env::args().count() != 2 {
            panic!("Wrong number of arguments...");
        }

        let binary = fs::read(env::args().last().unwrap());

        let max_ram: usize = (4096 - START_USABLE_RAM).into();

        if binary.is_err() {
            return ;
        }
        let len_bin = binary.iter().count();
        if len_bin > max_ram {
            panic!("File too large...");
        }
        &mut self.ram[START_USABLE_RAM.into()..(START_USABLE_RAM as usize + len_bin).into()].copy_from_slice(&binary.unwrap());
        // self.ram.iter().skip(START_USABLE_RAM - 1)

    }

}




fn main() {
    let mut chip = Chip8::new();
    // fs::read(argv[1);
    chip.load_rom();
    
    println!("Hello, world! {0}", chip.pc);
}
