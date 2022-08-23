pub mod controller;
pub mod sensors;
use controller::*;

use macroquad::prelude::*;
use sensors::*;

pub struct Car {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub width: f32,
    pub height: f32,
    pub max_speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
    pub angle: f32,
    pub sensors: Sensors,
    pub controlls: Controlls,
}

impl Car {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x: x,
            y: y,
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 3.0,
            rotation_speed: 0.04,
            friction: 0.05,
            width: width,
            height: height,
            angle: 0.0,
            sensors: Sensors::new(),
            controlls: Controlls::new(),
        }
    }
    pub fn update_car(&mut self) {
        self.move_car();
    }
    pub fn move_car(&mut self) {
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.controlls.right = true;
        } else {
            self.controlls.right = false;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.controlls.left = true;
        } else {
            self.controlls.left = false;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.controlls.forward = true;
        } else {
            self.controlls.forward = false;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.controlls.reverse = true;
        } else {
            self.controlls.reverse = false;
        }
        // println!("left: {}", self.controlls.left);
        // println!("right: {}", self.controlls.right);
        // println!("forward: {}", self.controlls.forward);
        // println!("reverse: {}", self.controlls.reverse);
        println!("rotation: {}", self.angle);

        // FORWARD BACKWARD SPEED
        if self.controlls.reverse {
            self.speed -= self.acceleration;
        }
        if self.controlls.forward {
            self.speed += self.acceleration;
        }

        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
        if self.speed < -self.max_speed / 2.0 {
            self.speed = -self.max_speed / 2.0;
        }
        if self.speed > 0.0 {
            self.speed -= self.friction;
        }
        if self.speed < 0.0 {
            self.speed += self.friction;
        }
        if self.speed.abs() < self.friction {
            self.speed = 0.0;
        }

        //TURNING

        if self.speed != 0.0 {
            // let flip: i8 = if self.speed > 0.0 { 1 } else { -1 };
            // let flip = 1.0;
            if self.controlls.left {
                self.angle += self.rotation_speed * (self.speed / self.max_speed);
            }
            if self.controlls.right {
                self.angle -= self.rotation_speed * (self.speed / self.max_speed);
            }
        }

        self.x -= self.angle.sin() * self.speed;
        self.y -= self.angle.cos() * self.speed;
    }
}
