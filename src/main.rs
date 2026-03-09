mod chip8;
mod interface;

// use std::fs;
// use std::env;
// use rfd::FileDialog;

fn print_ram(ram: &[u8]) {
    for byte in ram {
        println!("{0} ", byte);
    }
}

// use macroquad::miniquad::window;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};




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
    use chip8::Chip8;
    use interface::*;

    let mut chip = Chip8::new();
    let window_size = vec2(screen_width(), screen_height());
    let mut data: u32 = 5;
    
    chip.load_fonts();
    load_ui().await;
    
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
                    chip.load_rom(open_rom_dialog().unwrap());
                    chip.print_ram();
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
