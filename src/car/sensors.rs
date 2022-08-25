use crate::car::CarOptions;
use macroquad::prelude::*;

pub struct Sensors {
    car_options: CarOptions,
    ray_count: i8,
    ray_length: f32,
    ray_spread: f32,
    rays: Vec<Vec<Vec2>>,
    readings: Vec<Option<IntersectionResult>>,
}

impl Sensors {
    pub fn new(car: CarOptions) -> Self {
        Self {
            car_options: car,
            ray_count: 30,
            ray_length: 200.0,
            ray_spread: std::f32::consts::PI / 2.0,
            rays: Vec::new(),
            readings: Vec::new(),
        }
    }
    pub fn default() -> Self {
        Self {
            car_options: CarOptions::default(),
            ray_count: 3,
            ray_length: 150.0,
            ray_spread: std::f32::consts::PI * 2.0,
            rays: Vec::new(),
            readings: Vec::new(),
        }
    }
    pub fn update(&mut self, opts: CarOptions, road_borders: &Vec<Vec<Vec2>>) {
        self.car_options = opts;
        self.cast_rays();
        self.readings = Vec::new();
        for i in 0..self.rays.len() {
            self.readings.push(get_reading(&self.rays[i], road_borders));
        }
    }
    // fn get_reading(&mut self, ray: Vec<Vec2>, road_borders: Vec<Vec<Vec2>>) -> f32 {
    //     1.0
    // }
    fn cast_rays(&mut self) {
        self.rays = Vec::new();
        for i in 0..self.ray_count {
            let ray_angle = lerp(
                self.ray_spread / 2.0,
                -self.ray_spread / 2.0,
                i as f32 / (self.ray_count - 1) as f32,
            ) + self.car_options.angle;

            let start: Vec2 = Vec2::new(self.car_options.x + 20.0, self.car_options.y + 40.0);
            let end: Vec2 = Vec2::new(
                self.car_options.x - ray_angle.sin() * self.ray_length + 20.0,
                self.car_options.y - ray_angle.cos() * self.ray_length + 40.0,
            );

            self.rays.push(vec![start, end]);
        }
    }
    pub fn draw(&mut self) {
        for i in 0..self.rays.len() {
            let mut end: Vec2 = self.rays[i][1];
            match &self.readings[i] {
                Some(result) => {
                    end = Vec2::new(result.x, result.y);
                }
                None => (),
            }

            draw_line(
                self.rays[i][0].x,
                self.rays[i][0].y,
                end.x,
                end.y,
                3.0,
                YELLOW,
            );

            draw_line(
                self.rays[i][1].x,
                self.rays[i][1].y,
                end.x,
                end.y,
                3.0,
                BLACK,
            );
        }
    }
}
pub fn lerp(A: f32, B: f32, t: f32) -> f32 {
    return A + (B - A) * t;
}
fn get_reading(ray: &Vec<Vec2>, road_borders: &Vec<Vec<Vec2>>) -> Option<IntersectionResult> {
    let mut touches: Vec<IntersectionResult> = Vec::new();

    for border in road_borders {
        let touch: Option<IntersectionResult> =
            get_intersection(ray[0], ray[1], border[0], border[1]);
        match touch {
            Some(v) => {
                touches.push(v);
            }
            None => (),
        }
    }
    if touches.len() == 0 {
        return None;
    } else {
        // let offsets:Vec<f32> = touches.into_iter().map(|val| {return val.offset}).collect();
        // let mut min_value:f32 = 200.0;
        // for val in offsets{
        //     if val < min_value {min_value = val;}
        // }
        let min_touch: IntersectionResult = touches
            .into_iter()
            .min_by(|x, y| x.offset.total_cmp(&y.offset))?;
        return Some(min_touch);
    }
}

pub struct IntersectionResult {
    x: f32,
    y: f32,
    offset: f32,
}

pub fn get_intersection(A: Vec2, B: Vec2, C: Vec2, D: Vec2) -> Option<IntersectionResult> {
    let t_top = (D.x - C.x) * (A.y - C.y) - (D.y - C.y) * (A.x - C.x);
    let u_top = (C.y - A.y) * (A.x - B.x) - (C.x - A.x) * (A.y - B.y);
    let bottom = (D.y - C.y) * (B.x - A.x) - (D.x - C.x) * (B.y - A.y);

    if bottom != 0.0 {
        let t = t_top / bottom;
        let u = u_top / bottom;
        if t >= 0. && t <= 1. && u >= 0. && u <= 1. {
            return Some(IntersectionResult {
                x: lerp(A.x, B.x, t),
                y: lerp(A.y, B.y, t),
                offset: t,
            });
        };
    }

    return None;
}
