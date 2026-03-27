use core::time;
use std::{sync::{Arc, Mutex}};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};
use crate::chip8::{Chip8, TARGET_SCREEN_HEIGH, TARGET_SCREEN_WIDTH};

const TARGET_FPS: u64 = 60;
const FREQUENCY_MS: u64 = 1000 / TARGET_FPS;


struct Renderer<'a> {
	image: &'a Arc<Mutex<Image>>,
}

impl <'a>Renderer<'a> {
	pub fn new(shared_bufffer: &'a Arc<Mutex<Image>>) -> Self {
		Self {
			image: shared_bufffer,
		}
	}


	async fn chip8_emu_loop(&self) {
        use macroquad::prelude::*;
        let sixteen_millis = time::Duration::from_millis(FREQUENCY_MS);
		clear_background(WHITE);
		loop {
            {
                let shared_buffer = self.image.lock().unwrap();
                let texture = Texture2D::from_image(&shared_buffer);
    
                draw_texture(&texture, 0.0, 0.0, WHITE);
            }
            std::thread::sleep(sixteen_millis);
            next_frame().await;
		}

	}
}

pub fn gen_empty_rectangle(
    width: u16,
    height: u16,
    thickness: u16,
    color: Color,
) -> Image {
    let mut img = Image::gen_image_color(width, height, Color::new(0., 0., 0., 0.));
    let offset = 4;
    
    for y in 0 + offset..height - offset {
        for x in 0 + offset..width - offset {
            if x < thickness + offset
                || x >= width - offset - thickness
                || y < thickness + offset
                || y >= height - offset - thickness
            {
                img.set_pixel(x as u32, y as u32, color);
            }
        }
    }
    img
}

pub fn open_rom_dialog() -> Option<Vec<u8>> {
	use rfd::FileDialog;

    let file = FileDialog::new()
        .add_filter("Chip8 ROM", &["ch8", "rom", "bin"])
        .pick_file();

    if let Some(path) = file {
        Some(std::fs::read(path).unwrap())
    } else {
        None
    }
}

pub fn create_textures() -> (Image, Image, Image){
    let window_background = Image::gen_image_color(screen_width() as u16, screen_height() as u16, DARKGRAY);
    
    let empty_rec = gen_empty_rectangle(120, 60, 3, DARKPURPLE);
    let mut button_background = Image::gen_image_color(120, 60, DARKBLUE);
    button_background.overlay(&empty_rec);
    
    let empty_rec_clicked = gen_empty_rectangle(118, 58, 3, DARKPURPLE);
    let mut button_clicked_background = Image::gen_image_color(118, 58, BLUE);
    button_clicked_background.overlay(&empty_rec_clicked);

    (window_background, button_background, button_clicked_background)
}

pub async fn load_ui() {
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

fn error_popup(error: &str) {
    println!("error: {error}.");
}

fn launch_emu(shared_buffer: Arc<Mutex<Image>>) -> std::thread::JoinHandle<()> {
    let mut chip = Chip8::new(shared_buffer);
        
    let emu_thread_handle = std::thread::spawn(move || {
		
		let file = open_rom_dialog();
        if let Some(rom) = file {
			chip.init(rom);
			// chip.print_ram();
            chip.interpret();
            
        } else {
			error_popup("loading ROM file");
            return; // returns from closure, act as a continue here
        }

    });

    emu_thread_handle   
}

pub async fn main_loop() {
	let window_size = vec2(screen_width(), screen_height());
	let render_buffer = Arc::new(Mutex::new(Image::gen_image_color(TARGET_SCREEN_WIDTH as u16, TARGET_SCREEN_HEIGH as u16, WHITE)));
    let renderer = Renderer::new(&render_buffer);
	let emu_buffer = Arc::clone(&render_buffer);
    let mut state = false;
    let mut thread_handler: Option<std::thread::JoinHandle<()>> = None;

	
	while state == false {
		clear_background(GRAY);
		root_ui().window(
			hash!(),
			vec2(0.0, 0.0),
			window_size,
			|ui| {
				ui.label(vec2(screen_width() * 0.5, 10.0), "Main Menu");
				if ui.button(vec2(screen_width() * 0.5, 275.0), "launch ROM") {
					// TODO
                    state = true;
					thread_handler = Some(launch_emu(Arc::clone(&emu_buffer)));
				}
				if ui.button(vec2(screen_width() * 0.5, 350.0), "Quit") {
					std::process::exit(0);
				}
				if ui.button(vec2(5.0, screen_height() - 50.0), "settings") {
					// TODO
				}
			},
		);
		draw_rectangle_lines(005., 05., 100., 50.0, 10.0, RED);
		next_frame().await;
	}

	renderer.chip8_emu_loop().await;
    
    if thread_handler.is_some() {
        thread_handler.unwrap().join().unwrap();
    }
}
