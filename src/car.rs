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
    pub is_main_car: bool,
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
    pub polygon: Vec<Vec2>,
    pub damaged: bool,
}
impl CarOptions {
    pub fn from_car(car: &Car) -> Self {
        return car.opts.clone();
    }
    pub fn new(x: f32, y: f32, width: f32, height: f32, is_main_car: bool) -> Self {
        Self {
            is_main_car,
            x: x,
            y: y,
            speed: 0.0,
            acceleration: 0.2,
            max_speed: if is_main_car { 3.0 } else { 2.0 },
            rotation_speed: 0.03,
            friction: 0.05,
            width: width,
            height: height,
            angle: 0.0,
            controlls: Controlls::new(is_main_car),
            polygon: Vec::new(),
            damaged: false,
        }
    }
    pub fn default() -> Self {
        Self {
            is_main_car: false,
            x: 0.0,
            y: 0.0,
            speed: 0.0,
            acceleration: 0.2,
            max_speed: 1.0,
            rotation_speed: 0.03,
            friction: 0.05,
            width: 500.0,
            height: 500.0,
            angle: 0.0,
            controlls: Controlls::new(false),
            polygon: Vec::new(),
            damaged: false,
        }
    }
}

impl Car {
    pub fn new(x: f32, y: f32, width: f32, height: f32, is_main_car: bool) -> Self {
        let mut car = Car {
            opts: CarOptions::new(x, y, width, height, is_main_car),
            sensors: Sensors::default(),
        };
        if is_main_car {
            car.sensors = Sensors::new(car.opts.clone());
        }
        return car;
    }
    pub fn update(&mut self, road_borders: &Vec<Vec<Vec2>>, traffic: &Vec<Car>) {
        // if self.opts.damaged == false {
        self.move_car();
        self.opts.polygon = self.create_polygon();
        self.opts.damaged = self.assess_demage(road_borders, traffic);
        // }
        if self.sensors.active {
            self.sensors
                .update(self.opts.clone(), road_borders, traffic);
        }
    }
    fn assess_demage(&self, road_borders: &Vec<Vec<Vec2>>, traffic: &Vec<Car>) -> bool {
        for i in 0..traffic.len() {
            match get_poly_intersection(&self.opts.polygon, &traffic[i].opts.polygon) {
                true => {
                    return true;
                }
                false => (),
            }
        }

        for k in 0..road_borders.len() {
            match get_poly_intersection(&self.opts.polygon, &road_borders[k]) {
                true => {
                    return true;
                }
                false => (),
            }
        }

        return false;
    }
    fn create_polygon(&self) -> Vec<Vec2> {
        let mut points: Vec<Vec2> = Vec::new();
        let rad = ((self.opts.height * self.opts.height + self.opts.width * self.opts.width)
            as f64)
            .sqrt()
            / 2.0;
        let alpha = libm::atan2(self.opts.width as f64, self.opts.height as f64);
        points.push(Vec2::new(
            self.opts.x - (self.opts.angle - alpha as f32).sin() * rad as f32
                + self.opts.width / 2.0,
            self.opts.y - (self.opts.angle - alpha as f32).cos() * rad as f32
                + self.opts.height / 2.0,
        ));
        points.push(Vec2::new(
            self.opts.x - (self.opts.angle + alpha as f32).sin() * rad as f32
                + self.opts.width / 2.0,
            self.opts.y - (self.opts.angle + alpha as f32).cos() * rad as f32
                + self.opts.height / 2.0,
        ));
        points.push(Vec2::new(
            self.opts.x
                - (std::f32::consts::PI + self.opts.angle - alpha as f32).sin() * rad as f32
                + self.opts.width / 2.0,
            self.opts.y
                - (std::f32::consts::PI + self.opts.angle - alpha as f32).cos() * rad as f32
                + self.opts.height / 2.0,
        ));
        points.push(Vec2::new(
            self.opts.x
                - (std::f32::consts::PI + self.opts.angle + alpha as f32).sin() * rad as f32
                + self.opts.width / 2.0,
            self.opts.y
                - (std::f32::consts::PI + self.opts.angle + alpha as f32).cos() * rad as f32
                + self.opts.height / 2.0,
        ));
        return points;
    }
    pub fn move_car(&mut self) {
        if self.opts.controlls.active == false {
            self.opts.controlls.forward = true;
        } else {
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
        }
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
        for i in 0..self.opts.polygon.len() {
            draw_line(
                self.opts.polygon[i].x,
                self.opts.polygon[i].y,
                self.opts.polygon[(i + 1) % self.opts.polygon.len()].x,
                self.opts.polygon[(i + 1) % self.opts.polygon.len()].y,
                3.0,
                if self.opts.is_main_car { RED } else { BLUE },
            );
        }
        if self.opts.damaged {
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
        }
        if self.sensors.active {
            self.sensors.draw();
        }
    }
}
