pub mod car;

use car::*;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let texture: Texture2D = load_texture("assets/car.png").await.unwrap();

    let mut car: Car = Car::new(
        screen_width() / 2. - 20.,
        screen_height() / 2. - 40.,
        40.,
        80.,
    );

    loop {
        clear_background(WHITE);
        car.update_car();

        draw_rectangle(car.x, car.y, car.width, car.height, WHITE);
        draw_texture_ex(
            texture,
            car.x,
            car.y,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: -car.angle,
                pivot: None,
                flip_x: false,
                flip_y: false,
            },
        );

        // let rotation = car.angle.to_radians();
        // let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
