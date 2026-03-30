mod chip8;
mod interface;
use std::ops::Shr;

use macroquad::prelude::*;

use crate::chip8::{TARGET_SCREEN_HEIGH, TARGET_SCREEN_WIDTH};

fn conf() -> Conf {
	Conf {
	window_title: "emu".to_string(), //this field is not optional!
	fullscreen:false,
	window_resizable: false,
	window_width: TARGET_SCREEN_WIDTH as i32,
	window_height: TARGET_SCREEN_HEIGH as i32,
	..Default::default()
	}
}

#[macroquad::main(conf)]
async fn main() {
	use interface::*;

	// println!("{0}", (45 as i16 + 8 - 63));					
	use std::ops::BitAnd;
	let xor = 58;
	
	let reminder: i16 = match xor as i16 + 8 as i16 - 63 as i16 {
		x if x < 0 => 0,
        x => x
    };
	println!("{reminder}");
	for i in (reminder..8).rev() {
	   print!(" {0}", ((0b10101000 as u8).shr(i) as u8).bitand(1) == 1);
	}


	load_ui().await;
	main_loop().await;
}
