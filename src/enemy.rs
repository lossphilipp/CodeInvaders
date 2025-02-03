use macroquad::prelude::*;
use std::rc::Rc;

pub enum Direction {
    Left,
    Right,
}

pub struct Enemy {
    pub texture: Rc<Texture2D>,
    pub scale: f32,
    pub position: Vec2,
    pub level: i8,
    pub collided: bool,
    current_direction: Direction,
}

impl Enemy {
    pub fn new(texture: Rc<Texture2D>, scale: f32, level: i8, position: Vec2) -> Self {
        Self {
            texture,
            scale,
            position,
            level,
            collided: false,
            current_direction: Direction::Left,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.texture.width() * self.scale, self.texture.height() * self.scale)),
                ..Default::default()
            },
        );
    }

    fn change_direction(&mut self) {
        self.position.y += self.texture.height() * self.scale;

        self.current_direction = match self.current_direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }

    pub fn update(&mut self, direction_change: &bool, delta_time: &f32) {
        let base_speed: f32 = screen_width() / 25.0;
        let move_speed: f32 = base_speed * (1.5_f32).powi(self.level as i32);

        if *direction_change {
            self.change_direction();
        }

        match self.current_direction {
            Direction::Left => self.position.x += move_speed * *delta_time,
            Direction::Right => self.position.x -= move_speed * *delta_time,
        };
    }
}