#![allow(unused)]
use macroquad::prelude::*;
use std::f32::consts::PI;

const WINDOW_SIZE: f32 = 800.0;
const MOVE_SPEED: f32 = 5.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player {
        x: 300.0,
        y: 300.0,
        dx: f32::cos(0.0) * MOVE_SPEED,
        dy: f32::sin(0.0) * MOVE_SPEED,
        angle: 0.0,
    };
    let map = Map::new();

    loop {
        clear_background(DARKGRAY);

        map.draw_map();
        player.draw_player();

        player.move_player();

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    angle: f32,
}

impl Player {
    fn draw_player(&self) {
        let player_size = 8.0;

        draw_rectangle(self.x, self.y, player_size, player_size, YELLOW);
        draw_line(
            self.x + player_size / 2.0,
            self.y + player_size / 2.0,
            self.x + self.dx * MOVE_SPEED + player_size / 2.0,
            self.y + self.dy * MOVE_SPEED + player_size / 2.0,
            4.0,
            YELLOW,
        );
    }

    fn move_player(&mut self) {
        if is_key_down(KeyCode::A) {
            self.angle -= 0.1;
            if self.angle < 0.0 {
                self.angle += 2.0 * PI;
            }
            self.dx = f32::cos(self.angle) * MOVE_SPEED;
            self.dy = f32::sin(self.angle) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.angle += 0.1;
            if self.angle > 2.0 * PI {
                self.angle -= 2.0 * PI;
            }
            self.dx = f32::cos(self.angle) * MOVE_SPEED;
            self.dy = f32::sin(self.angle) * MOVE_SPEED;
        }
        if is_key_down(KeyCode::W) {
            self.x += self.dx;
            self.y += self.dy;
        }
        if is_key_down(KeyCode::S) {
            self.x -= self.dx;
            self.y -= self.dy;
        }
    }
}

struct Map {
    data: [i32; 64],
    size: f32,
    xy: f32,
    wall_size: f32,
}

impl Map {
    fn new() -> Self {
        #[rustfmt::skip]
        let data = [
            1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 1, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1,
        ];
        Self {
            data,
            size: 64.0,
            xy: 8.0,
            wall_size: 100.0,
        }
    }

    fn draw_map(&self) {
        let outline_size = 4.0;
        for x in 0..self.xy as usize {
            for y in 0..self.xy as usize {
                let mut color = BLACK;
                if self.data[y * self.xy as usize + x] == 1 {
                    color = WHITE;
                }
                draw_rectangle(
                    x as f32 * self.wall_size + outline_size,
                    y as f32 * self.wall_size + outline_size,
                    self.wall_size - outline_size,
                    self.wall_size - outline_size,
                    color,
                );
            }
        }
    }
}
