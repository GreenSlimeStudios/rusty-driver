// use crate::utils::*;
use macroquad::prelude::*;
const INFINITY: f32 = 1000000.0;

pub struct Road {
    pub x: f32,
    pub width: f32,
    pub lane_count: i8,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub borders: Vec<Vec<Vec2>>,
}

impl Road {
    pub fn new(x: f32, width: f32, lane_count: i8) -> Self {
        let mut road = Self {
            x,
            width,
            lane_count,

            left: x - width / 2.0,
            right: x + width / 2.0,

            top: -INFINITY,
            bottom: INFINITY,

            borders: Vec::new(),
        };
        let top_left: Vec2 = Vec2::new(road.left, road.top);
        let top_right: Vec2 = Vec2::new(road.right, road.top);
        let bottom_left: Vec2 = Vec2::new(road.left, road.bottom);
        let bottom_right: Vec2 = Vec2::new(road.right, road.bottom);

        road.borders.push(vec![top_left, bottom_left]);
        road.borders.push(vec![top_right, bottom_right]);

        println!("{:?}", road.borders);

        return road;
    }
    pub fn draw(&mut self, y: &f32) {
        // let margin: f32 = (screen_width() - self.width) / 2.0;

        // draw_line(self.left, 0.0, self.left, screen_height(), 10.0, RED);
        // draw_line(
        //     self.right,
        //     // screen_width() - margin,
        //     0.0,
        //     self.right,
        //     screen_height(),
        //     10.0,
        //     RED,
        // );
        // draw_rectangle(self.left, self.top, self.right, self.bottom, BEIGE);
        draw_rectangle(
            self.left - 30.0,
            y - screen_height(),
            (self.right - (700.0 - self.width) / 2.0) + 60.0,
            INFINITY,
            GRAY,
        );
        for i in 1..self.lane_count {
            let x = lerp(self.left, self.right, i as f32 / self.lane_count as f32);
            draw_line(x, self.top, x, self.bottom, 7.0, WHITE);
        }
        for i in 0..600 {
            draw_line(
                self.left + 30.0,
                -(i * 80) as f32 + screen_height(),
                self.right - 30.0,
                -(i * 80) as f32 + screen_height(),
                20.0,
                GRAY,
            )
        }

        for border in &self.borders {
            draw_line(border[0].x, border[0].y, border[1].x, border[1].y, 7.0, RED)
        }
    }
    pub fn get_lane_center(&mut self, lane_index: i8) -> f32 {
        let lane_width: f32 = self.width / self.lane_count as f32;
        return self.left + lane_width as f32 / 2.0 + lane_index as f32 * lane_width;
    }
}
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}
