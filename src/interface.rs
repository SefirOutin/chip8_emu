use macroquad::prelude::*;
use rfd::FileDialog;
use macroquad::ui::{hash, root_ui, Skin};


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
    let file = FileDialog::new()
        .add_filter("Chip8 ROM", &["ch8", "rom", "bin"])
        .pick_file();

    if let Some(path) = file {
        std::fs::read(path).ok()
    } else {
        None
    }
}

pub fn create_textures() -> (Image, Image, Image){
    let window_background = Image::gen_image_color(screen_width() as u16, screen_height() as u16, DARKGRAY);
    
    let empty_rec = gen_empty_rectangle(120, 60, 3, PURPLE);
    let mut button_background = Image::gen_image_color(120, 60, DARKBLUE);
    button_background.overlay(&empty_rec);
    
    let empty_rec_clicked = gen_empty_rectangle(118, 58, 3, PURPLE);
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

// async pub fn looop() {
//     loop 
// }