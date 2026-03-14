mod chip8;
mod interface;

use macroquad::prelude::*;

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
    
    chip.load_fonts();
    load_ui().await;
    main_loop(chip).await;
}
