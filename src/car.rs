pub mod controller;
pub mod sensors;

use controller::*;

use macroquad::prelude::*;
use sensors::*;

pub struct Car {
    pub opts: CarOptions,
    pub sensors: Sensors,
}
#[derive(Clone)]
pub struct CarOptions {
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
    pub controlls: Controlls,
}
impl CarOptions {
    pub fn from_car(car: &Car) -> Self {
        return car.opts.clone();
    }
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x: x,
            y: y,
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 3.0,
            rotation_speed: 0.03,
            friction: 0.05,
            width: width,
            height: height,
            angle: 0.0,
            controlls: Controlls::new(),
        }
    }
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 3.0,
            rotation_speed: 0.03,
            friction: 0.05,
            width: 500.0,
            height: 500.0,
            angle: 0.0,
            controlls: Controlls::new(),
        }
    }
}

impl Car {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let mut car = Car {
            opts: CarOptions::new(x, y, width, height),
            sensors: Sensors::default(),
        };
        car.sensors = Sensors::new(car.opts.clone());
        return car;
    }
    pub fn update(&mut self, road_borders: &Vec<Vec<Vec2>>) {
        self.move_car();
        self.sensors.update(self.opts.clone(), road_borders);
    }
    pub fn move_car(&mut self) {
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.opts.controlls.right = true;
        } else {
            self.opts.controlls.right = false;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.opts.controlls.left = true;
        } else {
            self.opts.controlls.left = false;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.opts.controlls.forward = true;
        } else {
            self.opts.controlls.forward = false;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.opts.controlls.reverse = true;
        } else {
            self.opts.controlls.reverse = false;
        }
        // println!("left: {}", self.options.controlls.left);
        // println!("right: {}", self.options.controlls.right);
        // println!("forward: {}", self.options.controlls.forward);
        // println!("reverse: {}", self.options.controlls.reverse);
        // println!("rotation: {}", self.options.angle);

        // FORWARD BACKWARD SPEED
        if self.opts.controlls.reverse {
            self.opts.speed -= self.opts.acceleration;
        }
        if self.opts.controlls.forward {
            self.opts.speed += self.opts.acceleration;
        }

        if self.opts.speed > self.opts.max_speed {
            self.opts.speed = self.opts.max_speed;
        }
        if self.opts.speed < -self.opts.max_speed / 2.0 {
            self.opts.speed = -self.opts.max_speed / 2.0;
        }
        if self.opts.speed > 0.0 {
            self.opts.speed -= self.opts.friction;
        }
        if self.opts.speed < 0.0 {
            self.opts.speed += self.opts.friction;
        }
        if self.opts.speed.abs() < self.opts.friction {
            self.opts.speed = 0.0;
        }

        //TURNING

        if self.opts.speed != 0.0 {
            // let flip: i8 = if self.options.speed > 0.0 { 1 } else { -1 };
            let flip = -1.0;
            if self.opts.controlls.left {
                self.opts.angle +=
                    flip * self.opts.rotation_speed * (self.opts.speed / self.opts.max_speed);
            }
            if self.opts.controlls.right {
                self.opts.angle -=
                    flip * self.opts.rotation_speed * (self.opts.speed / self.opts.max_speed);
            }
        }

        self.opts.x -= self.opts.angle.sin() * self.opts.speed;
        self.opts.y -= self.opts.angle.cos() * self.opts.speed;
    }
    pub fn draw(&mut self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.opts.x,
            self.opts.y,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: -self.opts.angle,
                pivot: None,
                flip_x: false,
                flip_y: false,
            },
        );
        self.sensors.draw();
    }
}
