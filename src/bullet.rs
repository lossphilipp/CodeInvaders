use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub collided: bool,
}

impl Bullet {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            collided: false,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 2.0, WHITE);
    }

    pub fn update(&mut self) {
        self.position.y -= 2.0;
    }
}