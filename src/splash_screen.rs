use image::{open, GenericImageView};
use minifb::{Key, Window, WindowOptions};

pub fn load_image_to_buffer(image_path: &str) -> (Vec<u32>, u32, u32) {
    let img = open(image_path).expect("Failed to load image");
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    let mut buffer: Vec<u32> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let color = (r << 16) | (g << 8) | b;
            buffer.push(color);
        }
    }
    (buffer, width, height)
}

pub fn show_splash_screen(image_path: &str) {
    let (buffer, width, height) = load_image_to_buffer(image_path);
    let mut window = Window::new(
        "Cat Maze - Press S to start",
        width as usize,
        height as usize,
        WindowOptions::default(),
    ).unwrap();

    window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();

    while window.is_open() && !window.is_key_down(Key::S) {
        window.update();
    }
}
