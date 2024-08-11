use minifb::{Window, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use crate::maze::is_wall;


pub struct Player {
    pub pos: Vec2,
    pub a: f32, //angle of view
    pub fov: f32, // field of view
}


pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0; 

    let mut new_pos_x = player.pos.x;
    let mut new_pos_y = player.pos.y;

    if window.is_key_down(Key::A){
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D){
        player.a += ROTATION_SPEED;
    }

    let forward_x = player.pos.x + MOVE_SPEED * player.a.cos();
    let forward_y = player.pos.y + MOVE_SPEED * player.a.sin();

    let backward_x = player.pos.x - MOVE_SPEED * player.a.cos();
    let backward_y = player.pos.y - MOVE_SPEED * player.a.sin();

    let strafe_left_x = player.pos.x + MOVE_SPEED * player.a.sin();
    let strafe_left_y = player.pos.y - MOVE_SPEED * player.a.cos();

    let strafe_right_x = player.pos.x - MOVE_SPEED * player.a.sin();
    let strafe_right_y = player.pos.y + MOVE_SPEED * player.a.cos();

    if window.is_key_down(Key::W) && !is_wall(maze, forward_x as usize, forward_y as usize) {
        new_pos_x = forward_x;
        new_pos_y = forward_y;
    }
    if window.is_key_down(Key::S) && !is_wall(maze, backward_x as usize, backward_y as usize) {
        new_pos_x = backward_x;
        new_pos_y = backward_y;
    }
    if window.is_key_down(Key::Q) && !is_wall(maze, strafe_left_x as usize, strafe_left_y as usize) {
        new_pos_x = strafe_left_x;
        new_pos_y = strafe_left_y;
    }
    if window.is_key_down(Key::E) && !is_wall(maze, strafe_right_x as usize, strafe_right_y as usize) {
        new_pos_x = strafe_right_x;
        new_pos_y = strafe_right_y;
    }

    player.pos.x = new_pos_x;
    player.pos.y = new_pos_y;
}