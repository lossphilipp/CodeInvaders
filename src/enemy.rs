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

    pub fn update(&mut self, direction_change: &bool) {
        let speed = 0.25;

        if *direction_change {
            self.change_direction();
        }

        match self.current_direction {
            Direction::Left => self.position.x += self.level as f32 * speed,
            Direction::Right => self.position.x -= self.level as f32 * speed,
        };
    }
}