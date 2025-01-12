use macroquad::prelude::*;

pub struct NameInput {
    pub name: String,
}

impl NameInput {
    pub fn new() -> Self {
        Self { name: String::new() }
    }

    pub fn update(&mut self) {
        if let Some(c) = get_char_pressed() {
            if c == '\n' {
                // Enter key pressed
                return;
            } else if c == '\u{8}' {
                // Backspace key pressed
                self.name.pop();
            } else {
                self.name.push(c);
            }
        }
    }

    pub fn draw(&self) {
        let prompt = "Enter your name:";
        let name = &self.name;
    
        draw_text(
            prompt,
            screen_width() / 2.0 - measure_text(prompt, None, 30, 1.0).width / 2.0,
            screen_height() / 2.0 - 20.0,
            30.0,
            LIGHTGRAY,
        );
    
        draw_text(
            name,
            screen_width() / 2.0 - measure_text(name, None, 30, 1.0).width / 2.0,
            screen_height() / 2.0 + 20.0,
            30.0,
            LIGHTGRAY,
        );
    }
}