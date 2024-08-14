mod maze;
mod framebuffer;
mod player;
use image::Frame;
use minifb::{Key, Window, WindowOptions};
use rodio::StreamError;
use core::f32::consts::PI;
use nalgebra_glm::{sqrt, Vec2, distance};
use player::{Player, process_events};
use std::{path::MAIN_SEPARATOR, time::Duration};
use framebuffer::Framebuffer;
use maze::load_maze;

use once_cell::sync::Lazy;
use std::sync::Arc; 

mod caster;
use caster::{cast_ray, cast_ray_minimap};

mod texture;
use texture::Texture;

mod splash_screen;

mod audio;

mod collectible;
use collectible::Collectible; 

static WALL: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall.png")));
static WALL1: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall1.png")));
static DOOR: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\door.png")));
static CAT: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\cat.png")));
static FISH1: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\fish.png")));
static FISH2: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\fish1.png")));



fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {

    let default_color = 0x000000;

    match cell {
        '+' => WALL1.get_pixel_color(tx, ty),
        '-' => WALL.get_pixel_color(tx, ty),
        '|' => WALL1.get_pixel_color(tx, ty),
        'g' => DOOR.get_pixel_color(tx, ty),
        _ => default_color,

    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+'  | '|' => WALL1.get_pixel_color(0, 0),  
        '-' => WALL.get_pixel_color(0, 0),
        'g' => 0xFF0000,                                
        _ => 0x717171,                                  
    };

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, z_buffer: &mut [f32]){

    let block_size = 100; 

    let hh =  framebuffer.height as f32/2.0;
    
    let num_rays = framebuffer.width; 

    
    for i in 0..framebuffer.width {
        framebuffer.set_current_color(0x818a6b);
        for j in 0..(framebuffer.height / 2){
            framebuffer.point(i, j);
        }
    
        framebuffer.set_current_color(0x182d10);
        for j in (framebuffer.height / 2)..framebuffer.height{
            framebuffer.point(i, j);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32/ num_rays as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);


        let distance = intersect.distance * (a - player.a).cos();

        let stake_height = (framebuffer.height as f32 / distance) * 50.0; 

        let stake_top = (hh - (stake_height / 2.0 )) as usize; 
        let stake_bottom = (hh + (stake_height / 2.0 )) as usize;

        z_buffer[i] = distance;

        for y in stake_top..stake_bottom{
            let ty = (y as f32- stake_top as f32) / (stake_bottom  as f32 - stake_top as f32) * 128.0;
            let tx = intersect.tx; 

            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }

}

fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2, z_buffer: &mut [f32]) {
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    let relative_angle = sprite_a - player.a;

    // Ajuste del ángulo relativo para mantenerlo dentro de -PI a PI
    let relative_angle = if relative_angle > PI {
        relative_angle - 2.0 * PI
    } else if relative_angle < -PI {
        relative_angle + 2.0 * PI
    } else {
        relative_angle
    };

    // Verificar si el sprite está dentro del campo de visión
    if relative_angle.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = distance(&player.pos, pos);

    if sprite_d < 10.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let start_x = (screen_width / 2.0) + (relative_angle * (screen_width / player.fov)) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width);
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height);
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                
                let color = CAT.get_pixel_color(tx, ty);
                if color != 0x443935 { 
                  framebuffer.set_current_color(color);
                  framebuffer.point(x, y);

                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, enemies: &[Vec2], z_buffer: &mut [f32]) {
    for enemy in enemies {
        render_enemy(framebuffer, player, enemy, z_buffer);
    }
}

fn render_collectible(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2, z_buffer: &mut [f32]) {
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    let relative_angle = sprite_a - player.a;

    // Ajuste del ángulo relativo para mantenerlo dentro de -PI a PI
    let relative_angle = if relative_angle > PI {
        relative_angle - 2.0 * PI
    } else if relative_angle < -PI {
        relative_angle + 2.0 * PI
    } else {
        relative_angle
    };

    // Verificar si el sprite está dentro del campo de visión
    if relative_angle.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = distance(&player.pos, pos);

    if sprite_d < 10.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 50.0;
    let start_x = (screen_width / 2.0) + (relative_angle * (screen_width / player.fov)) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width);
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height);
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                
                let color = FISH1.get_pixel_color(tx, ty);
                if color != 0x443935 { 
                  framebuffer.set_current_color(color);
                  framebuffer.point(x, y);

                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_collectibles(framebuffer: &mut Framebuffer, player: &Player, enemies: &[Vec2], z_buffer: &mut [f32]) {
    for collectible in enemies {
        render_collectible(framebuffer, player, collectible, z_buffer);
    }
}

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, minimap_x: usize, minimap_y: usize, minimap_scale: f32, collectibles: &[Vec2], enemies: &[Vec2]) {
    let block_size = (100.0 * minimap_scale) as usize; 

    // Dibujar el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let cell = maze[row][col];
            let xo = minimap_x + col * block_size;
            let yo = minimap_y + row * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Dibujar al jugador
    let player_x = minimap_x + (player.pos.x as f32 * minimap_scale) as usize;
    let player_y = minimap_y + (player.pos.y as f32 * minimap_scale) as usize;
    framebuffer.set_current_color(0xFF0000);  
    framebuffer.point(player_x, player_y);

    // Dibujar los coleccionables
    for collectible in collectibles {
        let collectible_x = minimap_x + (collectible.x * minimap_scale) as usize;
        let collectible_y = minimap_y + (collectible.y * minimap_scale) as usize;
        framebuffer.set_current_color(0xFF0000);  
        framebuffer.point(collectible_x, collectible_y);
    }
    
    // Dibujar enemigos
    for enemy in enemies {
        let enemy_x = minimap_x + (enemy.x * minimap_scale) as usize;
        let enemy_y = minimap_y + (enemy.y * minimap_scale) as usize;
        framebuffer.set_current_color(0xFFFFFF);  
        framebuffer.point(enemy_x, enemy_y);
    }
    
    let num_rays = 50;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray_minimap(framebuffer, &maze, player, angle, block_size, minimap_x, minimap_y, minimap_scale);
    }
}


fn main() {

    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);


    audio::play_background_music(); 
    splash_screen::show_splash_screen("src\\assets\\welcome1.png");

    let mut window = Window::new(
        "Rust Graphics - Cat Maze",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // move the window around
    window.set_position(100, 100);
    window.update();

    // initialize values
    framebuffer.set_background_color(0x333355);

    window.set_cursor_visibility(false); 

    let maze = load_maze("./maze.txt");

    let mut player = Player{
        pos: Vec2::new(150.0, 150.0),
        a: PI/1.3, 
        fov: PI/4.0,
    };

    let minimap_scale = 0.2;
    let minimap_width = (framebuffer.width as f32 * minimap_scale) as usize;
    let minimap_height = (framebuffer.height as f32 * minimap_scale) as usize;
    let minimap_x = framebuffer.width - minimap_width - 20;
    let minimap_y = framebuffer.height - minimap_height - 20;

    let enemies = vec![
        Vec2::new(260.0, 260.0),  
        Vec2::new(400.0, 400.0),
    ];

    let collectibles = vec![
        Vec2::new(500.0, 500.0),  
        Vec2::new(400.0, 400.0),
    ];

    while window.is_open(){
        //listen to inputs
        if window.is_key_down(Key::Escape){
            break;
        }

        process_events(&window, &mut player, &maze);

        framebuffer.clear();
    

        let mut z_buffer = vec![f32::INFINITY; framebuffer.width]; 

        render3d(&mut framebuffer, &player, &maze, &mut z_buffer); 
        render_enemies(&mut framebuffer, &player, &enemies, &mut z_buffer);
        render_collectibles(&mut framebuffer, &player, &collectibles, &mut z_buffer);

        render_minimap(&mut framebuffer, &player, &maze, minimap_x, minimap_y, minimap_scale, &collectibles, &enemies);

        // Actualizar la ventana con el buffer del framebuffer
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();

        std::thread::sleep(frame_delay);
    }
}


