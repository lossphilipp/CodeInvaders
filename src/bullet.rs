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
        // ToDo: Bullet size calculation creates a cheat: First start game & then resize window to make bullet size bigger
        draw_circle(self.position.x, self.position.y, screen_width() / 400.0, WHITE);
    }

    pub fn update(&mut self, delta_time: &f32) {
        let move_speed = screen_height() / 3.0;

        self.position.y -= move_speed * *delta_time;
    }
}