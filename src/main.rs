mod maze;
mod framebuffer;
mod player;
use minifb::{Key, Window, WindowOptions};
use core::{f32::consts::PI, num};
use nalgebra_glm::Vec2;
use player::{Player, process_events};
use std::{os::windows::io::FromRawHandle, process::Termination, time::Duration};
use framebuffer::Framebuffer;
use maze::load_maze;
use rusttype::{Font, Scale};
use image::{RgbaImage, Rgba};

use once_cell::sync::Lazy;
use std::sync::Arc; 

mod caster;
use caster::{cast_ray, Intersect};

mod texture;
use texture::Texture;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall1.png")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall1.png")));
static DOOR: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall2.png")));

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {

    let default_color = 0x000000;

    match cell {
        '+' => WALL1.get_pixel_color(tx, ty),
        '-' => WALL2.get_pixel_color(tx, ty),
        '|' => WALL2.get_pixel_color(tx, ty),
        'g' => DOOR.get_pixel_color(tx, ty),
        _ => default_color,

    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            if cell != ' ' {
                framebuffer.set_current_color(0xFFFFFF);
                framebuffer.point(x,y)
            }
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>){
    let block_size = 100; 

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize); 

    let num_rays = 100; 

    for i in 0..num_rays {
        let current_ray = i as f32/ num_rays as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        cast_ray(framebuffer, &maze, player, a, block_size, true); 
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>){

    let block_size = 100; 

    let hh =  framebuffer.height as f32/2.0;
    
    let num_rays = framebuffer.width; 

    
    for i in 0..framebuffer.width {

        framebuffer.set_current_color(0x383838);
        for j in 0..(framebuffer.height / 2){
            framebuffer.point(i, j);
        }
    
        framebuffer.set_current_color(0x717171);
        for j in (framebuffer.height / 2)..framebuffer.height{
            framebuffer.point(i, j);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32/ num_rays as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);


        let distance = intersect.distance * (a - player.a).cos();

        let stake_height = (framebuffer.height as f32 / distance) * 70.0; 

        let stake_top = (hh - (stake_height / 2.0 )) as usize; 
        let stake_bottom = (hh + (stake_height / 2.0 )) as usize;

        for y in stake_top..stake_bottom{
            let ty = (y as f32- stake_top as f32) / (stake_bottom  as f32 - stake_top as f32) * 128.0;
            let tx = intersect.tx; 

            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }

}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(100, 100);
    window.update();

    // initialize values
    framebuffer.set_background_color(0x333355);

    window.limit_update_rate(Some(Duration::from_millis(50))); // 20 fps
    window.set_cursor_visibility(false); // Ocultar el cursor

    let maze = load_maze("./maze.txt");

    let mut player = Player{
        pos: Vec2::new(150.0, 150.0),
        a: PI/3.0, 
        fov: PI/4.0,
    };

    let mut mode = "3D"; 
    let mut last_time = std::time::Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;

    while window.is_open(){

        let current_time = std::time::Instant::now();
        frame_count += 1;
        if current_time.duration_since(last_time).as_secs_f32() >= 1.0 {
            fps = frame_count as f32 / current_time.duration_since(last_time).as_secs_f32();
            frame_count = 0;
            last_time = current_time;
        }

        //listen to inputs
        if window.is_key_down(Key::Escape){
            break;
        }
    
        if window.is_key_down(Key::M){
            mode = if mode == "2D" {"3D"} else {"2D"}; 
        }

        process_events(&window, &mut player, &maze);

        framebuffer.clear();
    
        if mode == "2D"{
            render2d(&mut framebuffer, &player, &maze);
        } else {
            render3d(&mut framebuffer, &player, &maze)
        }

        window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();

        std::thread::sleep(frame_delay);
    }
}


