pub mod car;
pub mod road;

use car::*;
use road::*;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "rusty driver".to_owned(),
        // fullscreen: true,
        window_height: 1000,
        window_width: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    for i in 0..4000 {
        let rng = rand::gen_range(-100.0, 100.0);
    }

    let texture: Texture2D = load_texture("assets/car.png").await.unwrap();

    let mut road: Road = Road::new(700.0 / 2.0, 700.0 * 0.5, 4);
    let mut my_camera = Camera2D::default();
    my_camera.viewport = Some((
        0,
        0,
        screen_width().round() as i32,
        -screen_height().round() as i32,
    ));
    my_camera.zoom = Vec2::new(0.002, 0.002);

    // let mut car: Car = Car::new(
    //     road.get_lane_center(1) - 20.0,
    //     screen_height() / 2. - 40.,
    //     40.,
    //     80.,
    //     true,
    // );
    let mut cars: Vec<Car> = Vec::new();
    for i in 0..50 {
        cars.push(Car::new(
            road.get_lane_center(1) - 20.0,
            screen_height() / 2. - 40.,
            40.,
            80.,
            true,
        ));
    }
    let mut traffic: Vec<Car> = vec![Car::new(
        road.get_lane_center(1) - 20.0,
        -100.0,
        40.0,
        80.0,
        false,
    )];

    loop {
        let mut index: usize = 0;
        for i in 0..cars.len() {
            if cars[i].opts.y < cars[index].opts.y {
                index = i;
            }
        }

        for i in 0..traffic.len() {
            traffic[i].update(&road.borders, &Vec::new(), false);
        }

        my_camera.target = Vec2::new(100.0, cars[index].opts.y - 200.0);
        my_camera.rotation = 180.0;
        my_camera.viewport = Some((
            0,
            0,
            screen_height().round() as i32,
            screen_height().round() as i32,
        ));

        set_camera(&my_camera);

        clear_background(BLACK);

        for i in 0..cars.len() {
            if i == index {
                cars[i].update(&road.borders, &traffic, true);
            } else {
                cars[i].update(&road.borders, &traffic, false);
            }
        }
        // car.update(&vec![traffic[0].opts.polygon]);
        road.draw(&cars[index].opts.y);

        for i in 0..traffic.len() {
            traffic[i].draw(texture);
        }
        for i in 0..cars.len() {
            cars[i].draw(texture);
        }

        // set_camera(&Camera2D {
        //     zoom: vec2(1., screen_width() / screen_height()),
        //     ..Default::default() // offset: vec![0.0,car.y],
        // });

        // let rotation = car.angle.to_radians();
        // let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
