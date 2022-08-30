pub mod car;
pub mod road;

use car::network::*;
use car::*;
use road::*;

use macroquad::{
    prelude::{
        clear_background, is_key_down, load_texture, next_frame, screen_height, screen_width,
        set_camera, Camera2D, Conf, Texture2D, Vec2, BLACK, WHITE,
    },
    text::draw_text,
};
// use rand::thread_rng;
// use rand::Rng;
use rand::*;

const BATCH_SIZE: usize = 100;
const TRAFFIC_SIZE: usize = 50;

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
    let mut generation_count: u32 = 1;
    let mut show_sensors: bool = true;
    let texture: Texture2D = load_texture("assets/car.png").await.unwrap();

    let mut road: Road = Road::new(700.0 / 2.0, 320.0, 4);
    let mut my_camera = Camera2D::default();
    // my_camera.viewport = Some((
    //     0,
    //     0,
    //     screen_width().round() as i32,
    //     -screen_height().round() as i32,
    // ));
    my_camera.zoom = Vec2::new(0.002, 0.002);

    // let mut car: Car =
    let mut cars: Vec<Car> = Vec::new();
    for _i in 0..BATCH_SIZE {
        cars.push(Car::new(
            road.get_lane_center(1) - 20.0,
            -200.0,
            40.,
            80.,
            true,
        ));
    }
    let mut traffic: Vec<Car> = Vec::new();
    generate_traffic(&mut traffic, &mut road);
    let mut are_all_dmaged: bool = true;
    loop {
        if is_key_down(macroquad::prelude::KeyCode::E) {
            show_sensors = !show_sensors;
        }

        let mut index: usize = 0;
        for i in 0..cars.len() {
            if cars[i].opts.y < cars[index].opts.y {
                index = i;
            }
        }
        are_all_dmaged = true;
        for car in &cars {
            // println!("car {}", car.opts.damaged);
            if car.opts.damaged == false {
                are_all_dmaged = false;
            }
        }
        if is_key_down(macroquad::prelude::KeyCode::R) {
            are_all_dmaged = true;
        }
        // println!("overall {}", are_all_dmaged);
        if are_all_dmaged {
            let network: Network = cars[index].network.clone();
            cars.clear();
            for i in 0..BATCH_SIZE {
                cars.push(Car::new(
                    road.get_lane_center(1) - 20.0,
                    screen_height() / 2. - 40.,
                    40.,
                    80.,
                    true,
                ));
                cars[i].network = network.clone();
                cars[i].network.alter_values();
            }
            traffic.clear();
            traffic.push(Car::new(
                road.get_lane_center(1) - 20.0,
                -100.0,
                40.,
                80.,
                false,
            ));
            generate_traffic(&mut traffic, &mut road);
            generation_count += 1;
        }

        for i in 0..traffic.len() {
            traffic[i].update(&road.borders, &Vec::new(), false);
        }

        my_camera.target = Vec2::new(100.0, cars[index].opts.y - 200.0);
        my_camera.rotation = 180.0;
        // my_camera.world_to_screen(Vec2::new(0.0, cars[index].opts.y));
        my_camera.viewport = Some((
            0,
            0,
            // cars[index].opts.y.round() as i32,
            -screen_height().round() as i32,
            -screen_height().round() as i32,
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
            traffic[i].draw(texture, false, false);
        }
        for i in 0..cars.len() {
            if i == index {
                cars[i].draw(texture, true, true);
            } else {
                cars[i].draw(texture, false, show_sensors);
            }
        }

        for i in 0..cars.len() {
            if cars[i].opts.y > cars[index].opts.y + 300.0 {
                cars[i].opts.damaged = true;
            }
        }
        draw_text(
            generation_count.to_string().as_str(),
            -20.0,
            cars[index].opts.y - 500.0,
            22.0,
            WHITE,
        );

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

fn generate_traffic(traffic: &mut Vec<Car>, road: &mut Road) {
    for i in 0..TRAFFIC_SIZE {
        let road_index: i8 = rand::thread_rng().gen_range(0..4) as i8;

        traffic.push(Car::new(
            road.get_lane_center(road_index) as f32 - 20.0,
            -300.0 - 200.0 * i as f32,
            40.0,
            80.0,
            false,
        ));
        let road_index2: i8 = rand::thread_rng().gen_range(0..4) as i8;

        traffic.push(Car::new(
            road.get_lane_center(road_index2) as f32 - 20.0,
            -300.0 - 200.0 * i as f32,
            40.0,
            80.0,
            false,
        ));
        if (road_index - road_index2).abs() == 1 || (road_index2 - road_index).abs() == 1 {
            // println!("cars nearby");
            traffic.push(Car::new(
                road.get_lane_center(if road_index < road_index2 {
                    road_index
                } else {
                    road_index2
                }) as f32
                    - 20.0,
                -300.0 - 200.0 * i as f32,
                40.0 * 3.0,
                80.0,
                false,
            ));
        }
    }
}
