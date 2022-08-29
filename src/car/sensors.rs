use crate::car::Car;
use crate::car::CarOptions;
use macroquad::prelude::*;

pub struct Sensors {
    car_options: CarOptions,
    ray_count: i8,
    ray_length: f32,
    ray_spread: f32,
    rays: Vec<Vec<Vec2>>,
    pub readings: Vec<Option<IntersectionResult>>,
    pub active: bool,
}

impl Sensors {
    pub fn new(car: CarOptions) -> Self {
        Self {
            car_options: car,
            ray_count: 5,
            ray_length: 200.0,
            ray_spread: std::f32::consts::PI / 2.0,
            rays: Vec::new(),
            readings: Vec::new(),
            active: true,
        }
    }
    pub fn default() -> Self {
        Self {
            car_options: CarOptions::default(),
            ray_count: 3,
            ray_length: 0.0,
            ray_spread: std::f32::consts::PI * 2.0,
            rays: Vec::new(),
            readings: Vec::new(),
            active: false,
        }
    }
    pub fn update(&mut self, opts: CarOptions, road_borders: &Vec<Vec<Vec2>>, traffic: &Vec<Car>) {
        self.car_options = opts;
        self.cast_rays();
        self.readings = Vec::new();
        for i in 0..self.rays.len() {
            self.readings
                .push(get_reading(&self.rays[i], road_borders, traffic));
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

            let start: Vec2 = Vec2::new(
                self.car_options.x + self.car_options.width / 2.0,
                self.car_options.y + self.car_options.height / 2.0,
            );
            let end: Vec2 = Vec2::new(
                self.car_options.x - ray_angle.sin() * self.ray_length
                    + self.car_options.width / 2.0,
                self.car_options.y - ray_angle.cos() * self.ray_length
                    + self.car_options.height / 2.0,
            );

            self.rays.push(vec![start, end]);
        }
    }
    pub fn draw(&mut self, is_main_car: bool) {
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
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 0.0,
                    a: if is_main_car { 1.0 } else { 0.3 },
                },
            );

            draw_line(
                self.rays[i][1].x,
                self.rays[i][1].y,
                end.x,
                end.y,
                3.0,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: if is_main_car { 1.0 } else { 0.3 },
                },
            );
        }
    }
}
pub fn lerp(A: f32, B: f32, t: f32) -> f32 {
    return A + (B - A) * t;
}
fn get_reading(
    ray: &Vec<Vec2>,
    road_borders: &Vec<Vec<Vec2>>,
    traffic: &Vec<Car>,
) -> Option<IntersectionResult> {
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
    for d in 0..traffic.len() {
        let poly = &traffic[d].opts.polygon;
        for e in 0..poly.len() {
            match get_intersection(ray[0], ray[1], poly[e], poly[(e + 1) % poly.len()]) {
                Some(v) => {
                    touches.push(v);
                }
                None => (),
            }
        }
    }
    if touches.len() == 0 {
        return None;
    } else {
        return Some(
            touches
                .into_iter()
                .min_by(|a, b| a.offset.total_cmp(&b.offset))
                .unwrap(),
        );
    }
}
#[derive(Clone, Copy)]
pub struct IntersectionResult {
    x: f32,
    y: f32,
    pub offset: f32,
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

pub fn get_poly_intersection(poly1: &Vec<Vec2>, poly2: &Vec<Vec2>) -> bool {
    for i in 0..poly1.len() {
        for j in 0..poly2.len() {
            match get_intersection(
                poly1[i],
                poly1[(i + 1) % poly1.len()],
                poly2[j],
                poly2[(j + 1) % poly2.len()],
            ) {
                Some(_v) => {
                    return true;
                }
                None => (),
            }
        }
    }
    false
}
pub fn get_poly_intersection_detailes(
    poly1: &Vec<Vec2>,
    poly2: &Vec<Vec2>,
) -> Option<Vec<IntersectionResult>> {
    let mut touches: Vec<IntersectionResult> = Vec::new();
    for i in 0..poly1.len() {
        for j in 0..poly2.len() {
            match get_intersection(
                poly1[i],
                poly1[(i + 1) % poly1.len()],
                poly2[j],
                poly2[(j + 1) % poly2.len()],
            ) {
                Some(v) => {
                    touches.push(v);
                }
                None => (),
            }
        }
    }

    if touches.len() > 0 {
        return Some(touches);
    }
    None
}
