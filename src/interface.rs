use macroquad::ui::{hash, root_ui, Skin};

pub fn load_ui() {
	let window_background = load_image("window_background.png").await.unwrap();
    let button_background = load_image("button_background.png").await.unwrap();
    let button_clicked_background = load_image("button_clicked_background.png").await.unwrap();
    let font = load_file("atari_games.ttf").await.unwrap();

	 let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();
	
	let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(64)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();

	    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);
    let window_size = vec2(370.0, 320.0);
}

pub fn build_menu() {
	root_ui().window();
}