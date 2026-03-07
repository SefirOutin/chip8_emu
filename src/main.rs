use std::fs;
use std::env;

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

    pub fn load_fonts(&mut self) {
        let font_start_addr = 0x50;

        self.ram[font_start_addr..font_start_addr + FONT_SET_LEN].copy_from_slice(&FONT_SET);
    }

    pub fn load_rom(&mut self) {
        if env::args().count() != 2 {
            panic!("Wrong number of arguments...");
        }

        let binary = fs::read(env::args().last().unwrap());
        if binary.is_err() {
            panic!("error opening file...");
        }

        let len_bin = binary.as_ref().unwrap().len();
        if len_bin > AVAIBLE_RAM {
            panic!("File too large...");
        }

        self.ram[START_USABLE_RAM as usize..(START_USABLE_RAM as usize + len_bin )].clone_from_slice(&binary.unwrap());
    }

    pub fn interpret(&mut self) {

    }

}

fn print_ram(ram: &[u8]) {
    for byte in ram {
        println!("{0} ", byte);
    }
}

use macroquad::miniquad::window;
use macroquad::prelude::*;
// use macroquad::ui::root_ui;
use macroquad::ui::{hash, root_ui, Skin};


fn gen_empty_rectangle(
    width: u16,
    height: u16,
    thickness: u16,
    color: Color,
) -> Image {
    let mut img = Image::gen_image_color(width, height, Color::new(0., 0., 0., 0.));
    
    for y in 0..height {
        for x in 0..width {
            if x < thickness
                || x >= width - thickness
                || y < thickness
                || y >= height - thickness
            {
                img.set_pixel(x as u32, y as u32, color);
            }
        }
    }
    img
}

fn create_textures() -> (Image, Image, Image){
    let window_background = Image::gen_image_color(screen_width() as u16, screen_height() as u16, DARKGRAY);
    
    let empty_rec = gen_empty_rectangle(80, 30, 4, PURPLE);
    let mut button_background = Image::gen_image_color(80, 30, DARKBLUE);
    button_background.overlay(&empty_rec);
    
    let empty_rec_clicked = gen_empty_rectangle(79, 29, 4, PURPLE);
    let mut button_clicked_background = Image::gen_image_color(79, 29, BLUE);
    button_clicked_background.overlay(&empty_rec_clicked);

    (window_background, button_background, button_clicked_background)
}

async fn load_ui() {
    let (window_background, button_background, button_clicked_background) = create_textures();
    
    let font = load_file("ressources/SIXTY.TTF").await.unwrap();
    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        // .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        // .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
    let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(RED)
        .font_size(32)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(RED)
        .font_size(20)
        .build();
    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    
    root_ui().push_skin(&ui_skin);
}

fn conf() -> Conf {
 Conf {
   window_title: "emu".to_string(), //this field is not optional!
   fullscreen:false,
   window_resizable: false,
   //you can add other options too, or just use the default ones:
   ..Default::default()
 }
}

#[macroquad::main(conf)]
async fn main() {
    let mut chip = Chip8::new();
    chip.load_rom();
    chip.load_fonts();
    print_ram(&chip.ram);
    println!("Hello, world! {0}", chip.pc);
    
    load_ui().await;
    let window_size = vec2(screen_width(), screen_height());
    
    loop {
        clear_background(GRAY);
        root_ui().window(
            hash!(),
            vec2(0.0, 0.0),
            window_size,
            |ui| {
                ui.label(vec2(screen_width() * 0.5, 10.0), "Main Menu");
                if ui.button(vec2(screen_width() * 0.5, 275.0), "launch ROM") {
                    // TODO
                }
                if ui.button(vec2(screen_width() * 0.5, 350.0), "Quit") {
                    std::process::exit(0);
                }
                if ui.button(vec2(5.0, screen_height() - 50.0), "settings") {
                    // TODO
                }
            },
        );
        next_frame().await;
    }
}
