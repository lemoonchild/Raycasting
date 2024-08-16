use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{Window, Key, MouseMode};
use nalgebra_glm::{Vec2, rotate_vec2};
use std::f32::consts::PI;
use crate::maze::is_wall;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // Ángulo de visión
    pub fov: f32, // Campo de visión
    pub total_fishes: u32, // total de pescados capturados
    pub key_rendered: bool, // renderizado de la llave
    pub key_collected: bool, // obtener la llave del nivel
}

impl Player {
    pub fn process_events(&mut self, window: &Window, gilrs: &mut Gilrs, maze: &Vec<Vec<char>>) {
        const MOVE_SPEED: f32 = 5.0;
        const ROTATION_SPEED: f32 = 0.005;
        const STRAFE_SPEED: f32 = 10.0;

        // Procesamiento del ratón para la rotación
        let (mouse_x, _) = window.get_mouse_pos(MouseMode::Pass).unwrap_or((0.0, 0.0));
        unsafe {
            static mut LAST_MOUSE_X: f32 = 0.0;
            let mouse_dx = mouse_x - LAST_MOUSE_X;
            LAST_MOUSE_X = mouse_x;
            self.a += mouse_dx * ROTATION_SPEED;
        }

        // Procesamiento de teclado para el movimiento
        self.process_keyboard(window, maze, MOVE_SPEED, STRAFE_SPEED);

        // Procesamiento de eventos de mando
        self.process_gamepad(gilrs, maze, MOVE_SPEED);
    }

    fn move_if_possible(&mut self, movement: &Vec2, maze: &Vec<Vec<char>>) {
        let new_pos = self.pos + movement;
        if !is_wall(maze, new_pos.x as usize, new_pos.y as usize, self.key_collected) {
            self.pos = new_pos;
        }
    }    

    fn process_keyboard(&mut self, window: &Window, maze: &Vec<Vec<char>>, move_speed: f32, strafe_speed: f32) {
        let forward = Vec2::new(self.a.cos(), self.a.sin());
        let right = rotate_vec2(&forward, -PI / 2.0);

        if window.is_key_down(Key::W) {
            self.move_if_possible(&(forward * move_speed), maze);
        }
        if window.is_key_down(Key::S) {
            self.move_if_possible(&(forward * -move_speed), maze);
        }
        if window.is_key_down(Key::D) {
            self.move_if_possible(&(right * -strafe_speed), maze);
        }
        if window.is_key_down(Key::A) {
            self.move_if_possible(&(right * strafe_speed), maze);
        }
    
    }

    fn process_gamepad(&mut self, gilrs: &mut Gilrs, maze: &Vec<Vec<char>>, move_speed: f32) {
        const DEAD_ZONE: f32 = 0.1;  
        const ROTATION_SPEED_CONTROLLER: f32 = 0.05; 
    
        while let Some(event) = gilrs.next_event() {
            gilrs.update(&event);
            match event.event {
                EventType::AxisChanged(Axis::RightStickX, value, _) if value.abs() > DEAD_ZONE => {
                    
                    self.a += value * ROTATION_SPEED_CONTROLLER;
                },
                EventType::AxisChanged(Axis::LeftStickY, value, _) if value.abs() > DEAD_ZONE => {
                    let forward = Vec2::new(self.a.cos(), self.a.sin());
                    let movement = forward * (value * move_speed);
                    let new_pos = self.pos + movement;
                    if !is_wall(maze, new_pos.x as usize, new_pos.y as usize, self.key_collected) {
                        self.pos = new_pos; 
                    }
                },
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::DPadUp => {
                            let forward = Vec2::new(self.a.cos(), self.a.sin()) * move_speed;
                            let new_pos = self.pos + forward;
                            if !is_wall(maze, new_pos.x as usize, new_pos.y as usize, self.key_collected) {
                                self.pos = new_pos;
                            }
                        },
                        Button::DPadDown => {
                            let backward = Vec2::new(self.a.cos(), self.a.sin()) * -move_speed;
                            let new_pos = self.pos + backward;
                            if !is_wall(maze, new_pos.x as usize, new_pos.y as usize, self.key_collected) {
                                self.pos = new_pos;
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
    
}
