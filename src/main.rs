#![allow(unused)]
use macroquad::prelude::*;
use std::f32::consts::PI;

const ONE_DEGREE: f32 = 0.0174533;
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

        let mut angle = player.angle - PI / 6.0;
        for ray_num in 0..60 {
            let ray = Ray::new(&player, &map, angle);
            ray.draw_3d_wall(&map, ray_num as f32);
            angle += ONE_DEGREE;
        }

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
    distance: f32,
}

impl Ray {
    fn new(player: &Player, map: &Map, mut angle: f32) -> Self {
        let x = player.x;
        let y = player.y;

        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        if angle >= 2.0 * PI {
            angle -= 2.0 * PI;
        }

        let (vertical_intersection_x, vertical_intersection_y) =
            Self::get_vertical_ray(&player, &map, angle);
        let vertical_distance = ((x - vertical_intersection_x).powf(2.0)
            + (y - vertical_intersection_y).powf(2.0))
        .sqrt();

        let (horizontal_intersection_x, horizontal_intersection_y) =
            Self::get_horizontal_ray(&player, &map, angle);
        let horizontal_distance = ((x - horizontal_intersection_x).powf(2.0)
            + (y - horizontal_intersection_y).powf(2.0))
        .sqrt();

        let mut intersection_x: f32;
        let mut intersection_y: f32;
        if horizontal_distance > vertical_distance {
            intersection_x = vertical_intersection_x;
            intersection_y = vertical_intersection_y;
        } else {
            intersection_x = horizontal_intersection_x;
            intersection_y = horizontal_intersection_y;
        }

        Self {
            x: player.x,
            y: player.y,
            x1: intersection_x,
            y1: intersection_y,
            distance: f32::min(horizontal_distance, vertical_distance),
        }
    }

    fn draw_3d_wall(&self, map: &Map, ray_num: f32) {
        let wall_height = ((map.size * 320.0) / self.distance).clamp(0.0, 320.0);
        let wall_width = WINDOW_SIZE / 60.0;

        draw_line(
            ray_num * wall_width,
            WINDOW_SIZE - wall_height,
            ray_num * wall_width,
            wall_height,
            13.333,
            RED,
        )
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

    fn get_horizontal_ray(player: &Player, map: &Map, angle: f32) -> (f32, f32) {
        let x = player.x;
        let y = player.y;

        let mut intersection_y = 0.0;
        if angle > PI {
            intersection_y = ((y / map.wall_size).floor() * map.wall_size).abs();
        } else if angle < PI {
            intersection_y = (y / map.wall_size).floor() * map.wall_size + map.wall_size;
        }
        let mut intersection_x = x + (intersection_y - y) / angle.tan();

        for depth_of_field in 0..8 {
            let grid_x = (intersection_x / map.wall_size)
                .floor()
                .clamp(0.0, map.xy - 1.0) as usize;
            let mut grid_y = (intersection_y / map.wall_size)
                .floor()
                .clamp(0.0, map.xy - 1.0) as usize;

            if angle > PI && grid_y != 0 {
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
        }
        return (intersection_x as f32, intersection_y as f32);
    }
    fn get_vertical_ray(player: &Player, map: &Map, angle: f32) -> (f32, f32) {
        let x = player.x;
        let y = player.y;

        let mut intersection_x = 0.0;
        if angle > PI / 2.0 && angle < 3.0 * PI / 2.0 {
            intersection_x = ((x / map.wall_size).floor() * map.wall_size);
        } else if angle < PI / 2.0 || angle > 3.0 * PI / 2.0 {
            intersection_x = (x / map.wall_size).floor() * map.wall_size + map.wall_size;
        }
        let mut intersection_y = y + (intersection_x - x) * angle.tan();

        for depth_of_field in 0..8 {
            let mut grid_x = (intersection_x / map.wall_size)
                .floor()
                .clamp(0.0, map.xy - 1.0) as usize;
            let grid_y = (intersection_y / map.wall_size)
                .floor()
                .clamp(0.0, map.xy - 1.0) as usize;

            if grid_x != 0 && angle > PI / 2.0 && angle < 3.0 * PI / 2.0 {
                grid_x -= 1;
            }

            if map.data[grid_y * map.xy as usize + grid_x] == 1 {
                break;
            }

            if angle > PI / 2.0 && angle < 3.0 * PI / 2.0 {
                intersection_x -= map.wall_size;
                intersection_y -= map.wall_size * angle.tan();
            } else if angle < PI / 2.0 || angle > 3.0 * PI / 2.0 {
                intersection_x += map.wall_size;
                intersection_y += map.wall_size * angle.tan();
            }
        }

        return (intersection_x as f32, intersection_y as f32);
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
