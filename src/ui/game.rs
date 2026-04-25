use std::collections::VecDeque;
use macroquad::prelude::*;
use crate::ui::buttons::ButtonEvent;

const GRID_SIZE: i32 = 20;
const MOVE_INTERVAL: f32 = 0.15;
const CELL_GAP: f32 = 1.5;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    game_rect: Rect,
    snake: VecDeque<(i32, i32)>,
    direction: Direction,
    next_direction: Direction,
    food: (i32, i32),
    state: GameState,
    move_timer: f32,
    score: u32,
}

impl Game {
    pub fn new(game_rect: Rect) -> Self {
        let mid = GRID_SIZE / 2;
        let mut snake = VecDeque::new();
        snake.push_back((mid, mid));
        snake.push_back((mid - 1, mid));
        snake.push_back((mid - 2, mid));

        let mut game = Self {
            game_rect,
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: (0, 0),
            state: GameState::Playing,
            move_timer: 0.0,
            score: 0,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        loop {
            let x = macroquad::rand::gen_range(0, GRID_SIZE);
            let y = macroquad::rand::gen_range(0, GRID_SIZE);
            if !self.snake.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    pub fn handle_button(&mut self, event: ButtonEvent) {
        // Any button restarts when game is over
        if self.state == GameState::GameOver {
            let rect = self.game_rect;
            *self = Self::new(rect);
            return;
        }

        match event {
            ButtonEvent::Up => {
                if self.direction != Direction::Down {
                    self.next_direction = Direction::Up;
                }
            }
            ButtonEvent::Down => {
                if self.direction != Direction::Up {
                    self.next_direction = Direction::Down;
                }
            }
            ButtonEvent::Left => {
                if self.direction != Direction::Right {
                    self.next_direction = Direction::Left;
                }
            }
            ButtonEvent::Right => {
                if self.direction != Direction::Left {
                    self.next_direction = Direction::Right;
                }
            }
            ButtonEvent::A => {}
            ButtonEvent::B => {}
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.state != GameState::Playing {
            return;
        }

        self.move_timer += dt;
        if self.move_timer < MOVE_INTERVAL {
            return;
        }
        self.move_timer = 0.0;

        self.direction = self.next_direction;
        let (dx, dy) = self.direction.delta();
        let &head = self.snake.front().unwrap();
        let new_head = (head.0 + dx, head.1 + dy);

        // Wall collision
        if new_head.0 < 0
            || new_head.0 >= GRID_SIZE
            || new_head.1 < 0
            || new_head.1 >= GRID_SIZE
        {
            self.state = GameState::GameOver;
            return;
        }

        // Self collision
        if self.snake.contains(&new_head) {
            self.state = GameState::GameOver;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    pub fn draw(&self) {
        let cell = self.game_rect.w / GRID_SIZE as f32;

        // Background
        draw_rectangle(
            self.game_rect.x,
            self.game_rect.y,
            self.game_rect.w,
            self.game_rect.h,
            Color::from_rgba(10, 18, 10, 255),
        );

        // Subtle grid
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let x = self.game_rect.x + i as f32 * cell;
                let y = self.game_rect.y + j as f32 * cell;
                draw_rectangle_lines(x, y, cell, cell, 0.5, Color::from_rgba(25, 40, 25, 255));
            }
        }

        // Food
        {
            let (fx, fy) = self.food;
            let x = self.game_rect.x + fx as f32 * cell + CELL_GAP;
            let y = self.game_rect.y + fy as f32 * cell + CELL_GAP;
            let s = cell - CELL_GAP * 2.0;
            draw_rectangle(x, y, s, s, Color::from_rgba(220, 60, 60, 255));
        }

        // Snake
        for (i, &(gx, gy)) in self.snake.iter().enumerate() {
            let x = self.game_rect.x + gx as f32 * cell + CELL_GAP;
            let y = self.game_rect.y + gy as f32 * cell + CELL_GAP;
            let s = cell - CELL_GAP * 2.0;
            let color = if i == 0 {
                Color::from_rgba(120, 230, 120, 255) // head
            } else {
                Color::from_rgba(55, 165, 55, 255) // body
            };
            draw_rectangle(x, y, s, s, color);
        }

        // Score
        draw_text(
            &format!("Score: {}", self.score),
            self.game_rect.x + 6.0,
            self.game_rect.y + 20.0,
            20.0,
            Color::from_rgba(200, 200, 200, 200),
        );

        // Game over overlay
        if self.state == GameState::GameOver {
            draw_rectangle(
                self.game_rect.x,
                self.game_rect.y,
                self.game_rect.w,
                self.game_rect.h,
                Color::from_rgba(0, 0, 0, 160),
            );

            let cx = self.game_rect.x + self.game_rect.w / 2.0;
            let cy = self.game_rect.y + self.game_rect.h / 2.0;

            let title = "GAME OVER";
            let title_size = 42.0;
            let td = measure_text(title, None, title_size as u16, 1.0);
            draw_text(title, cx - td.width / 2.0, cy - 20.0, title_size, RED);

            let score_str = format!("Score: {}", self.score);
            let sd = measure_text(&score_str, None, 24, 1.0);
            draw_text(&score_str, cx - sd.width / 2.0, cy + 16.0, 24.0, WHITE);

            let hint = "Press any button to restart";
            let hd = measure_text(hint, None, 18, 1.0);
            draw_text(
                hint,
                cx - hd.width / 2.0,
                cy + 46.0,
                18.0,
                Color::from_rgba(180, 180, 180, 255),
            );
        }
    }
}
