const FONT_SET_LEN: usize = 80;
const FONT_SET: [u8;80] = [
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

const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 32;
const START_USABLE_RAM: u16 = 0x200;
const ALL_RAM: u16 = 4096;
const AVAIBLE_RAM: usize = (ALL_RAM - START_USABLE_RAM) as usize;


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
    pub fn new() -> Self {
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

    pub fn load_fonts(&mut self) {
        let font_start_addr = 0x50;

        self.ram[font_start_addr..font_start_addr + FONT_SET_LEN].copy_from_slice(&FONT_SET);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // let binary = fs::read(env::args().last().unwrap());
        // if binary.is_err() {
        //     panic!("error opening file...");
        // }

        let len_bin = rom.len();
        if len_bin > AVAIBLE_RAM {
            panic!("File too large...");
        }

        self.ram[START_USABLE_RAM as usize..(START_USABLE_RAM as usize + len_bin )].clone_from_slice(&rom);
    }

    pub fn interpret(&mut self) {

    }

	pub fn print_ram(&self) {
    for byte in self.ram {
        println!("{0} ", byte);
    }
}

}