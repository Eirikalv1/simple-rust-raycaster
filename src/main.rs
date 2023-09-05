#![allow(unused)]
use macroquad::prelude::*;
use std::f32::consts::PI;

const WINDOW_SIZE: f32 = 800.0;
const MOVE_SPEED: f32 = 5.0;
const PLAYER_SIZE: f32 = 8.0;
const WALL_OUTLINE_SIZE: f32 = 4.0;

#[macroquad::main(window_conf)]
async fn main() {
    let map = Map::new();
    let mut player = Player {
        x: 300.0,
        y: 300.0,
        dx: f32::cos(0.0) * MOVE_SPEED,
        dy: f32::sin(0.0) * MOVE_SPEED,
        angle: 0.0,
    };

    loop {
        clear_background(DARKGRAY);

        map.draw_map();
        player.draw_player();

        let ray = Ray::new(&player, &map);
        ray.draw_ray();

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

struct Ray {
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
}

impl Ray {
    fn new(player: &Player, map: &Map) -> Self {
        let x = player.x;
        let y = player.y;
        let angle = player.angle;

        // ---Check Horizontal Lines---
        /*let mut intersection_y = 0.0;
        if angle > PI {
            intersection_y = ((y / map.wall_size).floor() * map.wall_size).abs();
        } else if angle < PI {
            intersection_y = (y / map.wall_size).floor() * map.wall_size + map.wall_size;
        }
        let mut intersection_x = x + (intersection_y - y) / angle.tan();

        for depth_of_field in 0..8 {
            let grid_x = (intersection_x / map.wall_size).floor().clamp(0.0, map.xy - 1.0) as usize;
            let mut grid_y = (intersection_y / map.wall_size).floor().clamp(0.0, map.xy - 1.0) as usize;

            if angle > PI {
                grid_y -= 1;
            }

            if map.data[grid_y * map.xy as usize + grid_x] == 1 {
                break;
            }

            if angle > PI {
                intersection_x -= map.wall_size / angle.tan();
                intersection_y -= map.wall_size;
            } else if angle < PI {
                intersection_x += map.wall_size / angle.tan();
                intersection_y += map.wall_size;
            }
        }*/
        let mut intersection_x = 0.0;
        if angle > PI / 2.0 && angle < 3.0 * PI / 2.0 {
            intersection_x = ((x / map.wall_size).floor() * map.wall_size);
        } else if angle < PI / 2.0 || angle > 3.0 * PI / 2.0 {
            intersection_x = (x / map.wall_size).floor() * map.wall_size + map.wall_size;
        }

        let mut intersection_y = y + (intersection_x - x) * angle.tan();
        dbg!(intersection_x, intersection_y);
        /*
        for depth_of_field in 0..8 {
            let mut grid_x = (intersection_x / map.wall_size).floor().clamp(0.0, map.xy - 1.0) as usize;
            let grid_y = (intersection_y / map.wall_size).floor().clamp(0.0, map.xy - 1.0) as usize;

            if angle > PI / 2.0  {
                grid_x -= 1;
            }

            if map.data[grid_y * map.xy as usize + grid_x] == 1 {
                break;
            }

            if angle > PI / 2.0  {
                intersection_x -= map.wall_size;
                intersection_y -= map.wall_size / angle.tan();
            } else if angle < 3.0 * PI / 2.0 {
                intersection_x += map.wall_size;
                intersection_y += map.wall_size / angle.tan();
            }
        } */

        Self {
            x,
            y,
            x1: intersection_x,
            y1: intersection_y,
        }
    }
    fn draw_ray(&self) {
        draw_line(
            self.x + PLAYER_SIZE / 2.0,
            self.y + PLAYER_SIZE / 2.0,
            self.x1 + PLAYER_SIZE / 2.0,
            self.y1 + PLAYER_SIZE / 2.0 - WALL_OUTLINE_SIZE / 2.0,
            1.0,
            GREEN,
        );
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
        draw_rectangle(self.x, self.y, PLAYER_SIZE, PLAYER_SIZE, YELLOW);
        draw_line(
            self.x + PLAYER_SIZE / 2.0,
            self.y + PLAYER_SIZE / 2.0,
            self.x + self.dx * MOVE_SPEED + PLAYER_SIZE / 2.0,
            self.y + self.dy * MOVE_SPEED + PLAYER_SIZE / 2.0,
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
            if self.angle >= 2.0 * PI {
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
        for x in 0..self.xy as usize {
            for y in 0..self.xy as usize {
                let mut color = BLACK;
                if self.data[y * self.xy as usize + x] == 1 {
                    color = WHITE;
                }
                draw_rectangle(
                    x as f32 * self.wall_size + WALL_OUTLINE_SIZE,
                    y as f32 * self.wall_size + WALL_OUTLINE_SIZE,
                    self.wall_size - WALL_OUTLINE_SIZE,
                    self.wall_size - WALL_OUTLINE_SIZE,
                    color,
                );
            }
        }
    }
}
