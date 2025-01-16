use macroquad::prelude::*;

pub struct Player {
    pub texture: Texture2D,
    pub scale: f32,
    pub position: Vec2,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        let scale = screen_width() / 15.0 / texture.width();

        let position = vec2(
            screen_width() / 2.0 - (texture.width() * scale) / 2.0,
            screen_height() - (texture.height() * scale) - 10.0,
        );

        Self {
            texture,
            scale,
            position,
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

    pub fn reset(&mut self) {
        self.scale = screen_width() / 15.0 / self.texture.width();

        self.position = vec2(
            screen_width() / 2.0 - (self.texture.width() * self.scale) / 2.0,
            screen_height() - (self.texture.height() * self.scale) - 10.0,
        );
    }

    pub fn update(&mut self, delta_time: &f32) {
        let move_speed = screen_width() / 3.0;
        let left_bound = 10.0;
        let right_bound = screen_width() - (self.texture.width() * self.scale) - 10.0;

        if is_key_down(KeyCode::Left) {
            self.position.x -= move_speed * *delta_time;
            if self.position.x < left_bound {
                self.position.x = left_bound;
            }
        }
        if is_key_down(KeyCode::Right) {
            self.position.x += move_speed * *delta_time;
            if self.position.x > right_bound {
                self.position.x = right_bound;
            }
        }
    }
}