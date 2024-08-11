use minifb::{Window, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;


pub struct Player {
    pub pos: Vec2,
    pub a: f32, //angle of view
    pub fov: f32, // field of view
}


pub fn process_events(window: &Window, player: &mut Player){
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0; 


    // Rotación con las teclas A y D
    if window.is_key_down(Key::A){
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D){
        player.a += ROTATION_SPEED;
    }

    // Movimiento hacia adelante y hacia atrás con las teclas W y S

    if window.is_key_down(Key::W){
        player.pos.x = player.pos.x + MOVE_SPEED * player.a.cos();
        player.pos.y = player.pos.y + MOVE_SPEED * player.a.sin(); 
    }
    if window.is_key_down(Key::S){
        player.pos.x = player.pos.x - MOVE_SPEED * player.a.cos();
        player.pos.y = player.pos.y - MOVE_SPEED * player.a.sin(); 
    }

    // Movimiento lateral (strafe) con las teclas Q y E
    if window.is_key_down(Key::Q) {
        player.pos.x += MOVE_SPEED * player.a.sin();
        player.pos.y -= MOVE_SPEED * player.a.cos();
    }
    if window.is_key_down(Key::E) {
        player.pos.x -= MOVE_SPEED * player.a.sin();
        player.pos.y += MOVE_SPEED * player.a.cos();
    }
}