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
}

impl Road {
    pub fn new(x: f32, width: f32, lane_count: i8) -> Self {
        Self {
            x,
            width,
            lane_count,

            left: x - width / 2.0,
            right: x + width / 2.0,

            top: -INFINITY,
            bottom: INFINITY,
        }
    }
    pub fn draw(&mut self) {
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
            0.0,
            (self.right - (screen_width() - self.width) / 2.0) + 60.0,
            self.bottom,
            GRAY,
        );
        for i in 0..=self.lane_count {
            let x = lerp(self.left, self.right, i as f32 / self.lane_count as f32);
            draw_line(x, self.top, x, self.bottom, 7.0, WHITE);
        }
        for i in 0..((screen_height() / 80.0).round() as i32) {
            draw_line(
                self.left + 30.0,
                (i * 80) as f32,
                self.right - 30.0,
                (i * 80) as f32,
                20.0,
                GRAY,
            )
        }
    }
    pub fn get_lane_center(&mut self, lane_index: i8) -> f32 {
        let lane_width: f32 = self.width / self.lane_count as f32;
        return self.left + lane_width as f32 / 2.0 + lane_index as f32 * lane_width;
    }
}

fn lerp(A: f32, B: f32, t: f32) -> f32 {
    return A + (B - A) * t;
}
