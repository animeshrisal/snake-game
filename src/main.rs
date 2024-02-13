use macroquad::prelude::*;

const GRID_SIZE: i16 = 16;

type Point = (i16, i16);

const SPEED: i32 = 10;

struct Snake {
    dir: Point,
    body: Vec<Point>,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut fruit: Point = (rand::gen_range(0, GRID_SIZE), rand::gen_range(0, GRID_SIZE));
    let mut score: i32 = 0;
    let mut game_over: i32 = 0;
    let mut speed_counter: i32 = 0;

    let mut snake = Snake {
        dir: (0, 1),
        body: vec![(0, 0)],
    };

    loop {
        speed_counter += 1;

        if is_key_down(KeyCode::Right) {
            snake.dir = (1, 0)
        } else if is_key_down(KeyCode::Down) {
            snake.dir = (0, 1)
        } else if is_key_down(KeyCode::Left) {
            snake.dir = (-1, 0)
        } else if is_key_down(KeyCode::Up) {
            snake.dir = (0, -1)
        }

        let new_head: Point = (snake.body[0].0 + snake.dir.0, snake.body[0].1 + snake.dir.1);
        snake.body.insert(0, new_head);
        snake.body.pop();
        if snake.body[0] == fruit {
            snake.body.insert(0, fruit);
            fruit = (rand::gen_range(0, GRID_SIZE), rand::gen_range(0, GRID_SIZE));
        }

        clear_background(BLACK);

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                draw_rectangle_lines(20.0 * x as f32, 20.0 * y as f32, 20.0, 20.0, 1.0, RED)
            }
        }

        for _x in &mut snake.body {
            draw_rectangle(20.0 * _x.0 as f32, 20.0 * _x.1 as f32, 20.0, 20.0, WHITE);
        }

        draw_rectangle(
            20.0 * fruit.0 as f32,
            20.0 * fruit.1 as f32,
            20.0,
            20.0,
            YELLOW,
        );
        next_frame().await
    }
}
