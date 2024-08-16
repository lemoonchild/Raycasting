mod maze;
use maze::load_maze;

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::Player;

use minifb::{Key, Window, WindowOptions};
use splash_screen:: show_start_screen  ;
use core::f32::consts::PI;

use nalgebra_glm::{Vec2, distance};
use std::time::Duration;

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

mod textrender;
use textrender::TextRenderer; 

use std::time::Instant;

use gilrs::Gilrs;


// Imagenes del juego
static WALL: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\walls\\wall.png")));
static WALL1: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\walls\\wall1.png")));
static CAT: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\cat.png")));
static FISH1: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\fish.png")));
static KEY: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\key.png")));


fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {

    let default_color = 0x000000;

    match cell {
        '+' => WALL1.get_pixel_color(tx, ty),
        '-' => WALL.get_pixel_color(tx, ty),
        '|' => WALL1.get_pixel_color(tx, ty),
        'g' => WALL.get_pixel_color(tx, ty),
        _ => default_color,

    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+'  | '|' => WALL1.get_pixel_color(0, 0),  
        '-' => WALL.get_pixel_color(0, 0),
        'g' => WALL.get_pixel_color(0, 0),                                
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

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width) - 1;
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height) - 1;
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

fn render_collectible(framebuffer: &mut Framebuffer, player: &Player, collectible: &Collectible, z_buffer: &mut [f32]) {

    if collectible.collected {
        return;  
    }

    let pos = &collectible.position;
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    let relative_angle = sprite_a - player.a;

    let relative_angle = if relative_angle > PI {
        relative_angle - 2.0 * PI
    } else if relative_angle < -PI {
        relative_angle + 2.0 * PI
    } else {
        relative_angle
    };

    if relative_angle.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = distance(&player.pos, pos);

    if sprite_d < 10.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 20.0;
    let start_x = (screen_width / 2.0) + (relative_angle * (screen_width / player.fov)) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width) - 1;
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height) - 1;
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                
                let color = FISH1.get_pixel_color(tx, ty);
                if color != 0x6598b3 { 
                  framebuffer.set_current_color(color);
                  framebuffer.point(x, y);

                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_collectibles(framebuffer: &mut Framebuffer, player: &Player, collectibles: &[Collectible], z_buffer: &mut [f32]) {
    for collectible in collectibles {
        render_collectible(framebuffer, player, collectible, z_buffer);
    }
}

fn update_collectibles(player: &mut Player, collectibles: &mut Vec<Collectible>) {
    let capture_distance = 10.0; 

    for collectible in collectibles.iter_mut() {
        if !collectible.collected && nalgebra_glm::distance(&player.pos, &collectible.position) < capture_distance {
            collectible.collected = true;
            player.total_fishes += 1;
        }
    }
}

fn render_key(framebuffer: &mut Framebuffer, player: &Player, key_position: &Vec2, z_buffer: &mut [f32], texture: &Arc<Texture>, is_rendered: bool) {
    if !is_rendered {
        return;
    }

    let sprite_a = (key_position.y - player.pos.y).atan2(key_position.x - player.pos.x);
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

    let sprite_d = distance(&player.pos, key_position);

    if sprite_d < 10.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 40.0;
    let start_x = (screen_width / 2.0) + (relative_angle * (screen_width / player.fov)) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width) - 1;
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height) - 1;
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                
                let color = texture.get_pixel_color(tx, ty);
                if color != 0xFFFFFF {  // Asume que el color blanco es transparente, ajusta según tu textura
                    framebuffer.set_current_color(color);
                    framebuffer.point(x, y);
                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, minimap_x: usize, minimap_y: usize, minimap_scale: f32, collectibles: &[Collectible], enemies: &[Vec2]) {
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
    framebuffer.set_current_color(0x00A6FF);  
    framebuffer.point(player_x, player_y);

    // Dibujar los coleccionables
    for collectible in collectibles {
        if !collectible.collected {
            let collectible_x = minimap_x + (collectible.position.x * minimap_scale) as usize;
            let collectible_y = minimap_y + (collectible.position.y * minimap_scale) as usize;
            framebuffer.set_current_color(0xFFFFFF);
            framebuffer.point(collectible_x, collectible_y);
        }
    }

    // Dibujar enemigos
    for enemy in enemies {
        let enemy_x = minimap_x + (enemy.x * minimap_scale) as usize;
        let enemy_y = minimap_y + (enemy.y * minimap_scale) as usize;
        framebuffer.set_current_color(0xD32C2C);  
        framebuffer.point(enemy_x, enemy_y);
    }
    
    let num_rays = 20;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray_minimap(framebuffer, &maze, player, angle, block_size, minimap_x, minimap_y, minimap_scale);
    }
}

fn update_game_state(player: &mut Player, collectibles: &mut Vec<Collectible>, key_position: &Vec2, game_state: &mut String) {
    let capture_distance = 10.0;

    // Actualizar coleccionables
    for collectible in collectibles.iter_mut() {
        if !collectible.collected && nalgebra_glm::distance(&player.pos, &collectible.position) < capture_distance {
            collectible.collected = true;
            player.total_fishes += 1;
        }
    }

    // Chequear si todos los pescados han sido recolectados para renderizar la llave
    if player.total_fishes == collectibles.len() as u32 {
        player.key_rendered = true;
    }

    // Recolectar la llave si el jugador está cerca y la llave está renderizada
    if player.key_rendered && nalgebra_glm::distance(&player.pos, key_position) < capture_distance {
        player.key_collected = true;
        player.key_rendered = false; 
        *game_state = "SUCCESS".to_string();  
    }

}

fn main() {

    let font_data = std::fs::read("src\\assets\\fonts\\Montserrat-Medium.ttf").expect("failed to read font file");
    let text_renderer = TextRenderer::new(&font_data, 24.0);  // Ajusta el tamaño según necesites

    let mut gilrs = Gilrs::new().unwrap();

    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let key_position = Vec2::new(791.0, 250.0);  // Posición fija para la llave

    audio::play_background_music(); 
    show_start_screen("src\\assets\\screens\\welcome1.png");

    let mut game_state = "PLAY".to_string(); 

    let mut window = Window::new(
        "Rust Graphics - FEED THE CAT",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut last_frame_time = Instant::now();

    // move the window around
    window.set_position(100, 100);
    window.update();

    // initialize values
    framebuffer.set_background_color(0x333355);

    window.set_cursor_visibility(false); 

    let maze = load_maze("./maze.txt");

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI/1.3,
        fov: PI/4.0,
        total_fishes: 0,
        key_rendered: false,
        key_collected: false, 
    };    

    let minimap_scale = 0.2;
    let minimap_width = (framebuffer.width as f32 * minimap_scale) as usize;
    let minimap_height = (framebuffer.height as f32 * minimap_scale) as usize;
    let minimap_x = framebuffer.width - minimap_width - 20;
    let minimap_y = framebuffer.height - minimap_height - 20;

    let enemies = vec![
        Vec2::new(260.0, 260.0),  
        Vec2::new(178.0, 717.0),
        Vec2::new(1008.0, 155.0),
        Vec2::new(480.0, 329.0),
        Vec2::new(1096.0, 558.0),
    ];

    let mut collectibles: Vec<Collectible> = vec![
        Collectible::new(691.40, 753.22, Arc::clone(&FISH1)),
        Collectible::new(656.10, 164.82, Arc::clone(&FISH1)),
        Collectible::new(1107.38, 239.40, Arc::clone(&FISH1)),
        Collectible::new(833.94, 515.87, Arc::clone(&FISH1)),
    ];
    

    while window.is_open(){
        //listen to inputs
        if window.is_key_down(Key::Escape){
            break;
        }

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);

        last_frame_time = now;
        let fps = 1.0 / delta_time.as_secs_f32();
        let fps_text = format!("FPS: {:.2}", fps);
    
    
        let width = framebuffer.width;

        player.process_events(&window, &mut gilrs, &maze);
        update_collectibles(&mut player, &mut collectibles);
        
        framebuffer.clear();
    
        let mut z_buffer = vec![f32::INFINITY; framebuffer.width]; 

        match game_state.as_str() {
            "PLAY" => {
                render3d(&mut framebuffer, &player, &maze, &mut z_buffer); 
                render_enemies(&mut framebuffer, &player, &enemies, &mut z_buffer);
                render_collectibles(&mut framebuffer, &player, &collectibles, &mut z_buffer);
                render_minimap(&mut framebuffer, &player, &maze, minimap_x, minimap_y, minimap_scale, &collectibles, &enemies);
                text_renderer.render_text(&mut framebuffer, &fps_text, width as f32 - 150.0, 20.0, 0xFFFFFF);
        
                update_game_state(&mut player, &mut collectibles, &key_position, &mut game_state);
                let fish_text = format!("Pescados capturados: {}/4", player.total_fishes);
                text_renderer.render_text(&mut framebuffer, &fish_text, 20.0, 20.0, 0xFFFFFF);
            
                if player.key_rendered {
                    let action_text = "¡Has alimentado al gato! Recolecta la llave para pasar de nivel.";
                    text_renderer.render_text(&mut framebuffer, action_text, 400.0, 100.0, 0xFFFFFF);
                    let key_position = Vec2::new(791.0, 250.0);  
                    render_key(&mut framebuffer, &player, &key_position, &mut z_buffer, &KEY, player.key_rendered);
                }
            },
            "SUCCESS" => {
                framebuffer.set_background_color(0x000000); 
                text_renderer.render_text(&mut framebuffer, "¡Felicidades has alimentado al gato! Presiona \"E\" para salir del juego.", 100.0, 100.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, "¡Gracias por jugar!", 100.0, 130.0, 0xFFFFFF);

                if window.is_key_down(Key::E) {
                    break; 
                }
            },
            _ => {}
        }
        
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();

        std::thread::sleep(frame_delay);
    }
    
}


