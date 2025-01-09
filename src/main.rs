use macroquad::prelude::*;
mod player;
mod enemy;
mod bullet;
use player::Player;
use enemy::Enemy;
use bullet::Bullet;

enum GameState {
    Menu,
    Playing,
    LevelComplete,
    GameOver,
}

struct MenuText {
    text: String,
    font_size: u16,
}

async fn draw_menu(texts: Vec<MenuText>) {
    let padding = 10.0;
    let mut total_height = 0.0;

    // Calculate total height of all text items including padding
    for menu_text in &texts {
        let text_size = measure_text(&menu_text.text, None, menu_text.font_size, 1.0);
        total_height += text_size.height + padding;
    }
    total_height -= padding; // Remove the padding after the last item

    // Calculate starting y position to center all text items
    let mut current_y = screen_height() / 2.0 - total_height / 2.0;

    // Draw each text item
    for menu_text in &texts {
        let text_size = measure_text(&menu_text.text, None, menu_text.font_size, 1.0);
        draw_text(
            &menu_text.text,
            screen_width() / 2.0 - text_size.width / 2.0,
            current_y,
            menu_text.font_size as f32,
            LIGHTGRAY,
        );
        current_y += text_size.height + padding;
    }
}

async fn spawn_enemies(enemies: &mut Vec<Enemy>, level: &mut i8, rows: i8) {
    let texture = load_enemy_texture(*level).await; 

    let scale = screen_width() / 20.0 / texture.width();
    let enemy_width = texture.width() * scale;
    let enemy_height = texture.height() * scale;
    let spacing_x = enemy_width + 10.0;
    let spacing_y = enemy_height + 10.0;

    for row in 0..rows {
        for col in 0..10 {
            let x = col as f32 * spacing_x + 11.0;
            let y = row as f32 * spacing_y + 10.0;
            // ToDo: Optimize this by not cloning the texture for every enemy
            enemies.push(Enemy::new(
                Texture2D::clone(&texture),
                scale,
                *level,
                vec2(x, y),
            ));
        }
    }
}

async fn init_game (
    level: &mut i8,
    player: &mut Player,
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    game_state: &mut GameState,
) {
    *level += 1;

    player.reset_position();

    bullets.clear();

    enemies.clear();
    spawn_enemies(enemies, level, 5).await;

    *game_state = GameState::Playing;
}

async fn load_enemy_texture(level: i8) -> Texture2D {
    match level {
        1 => load_texture("assets/python.png").await.unwrap(),
        2 => load_texture("assets/java.png").await.unwrap(),
        3 => load_texture("assets/dart.png").await.unwrap(),
        4 => load_texture("assets/cplusplus.png").await.unwrap(),
        _ => load_texture("assets/c.png").await.unwrap(),
    }
}

async fn calculate_enemy_movement(enemies: &mut Vec<Enemy>) {
    let screen_width = screen_width();
    let mut hit_wall = false;

    let first_enemy = enemies.first().unwrap();
    let enemy_width = first_enemy.texture.width() * first_enemy.scale;

    for enemy in enemies.iter() {
        if enemy.position.x + enemy_width >= screen_width - 10.0 || enemy.position.x <= 10.0 {
            hit_wall = true;
            break;
        }
    }

    for enemy in enemies.iter_mut() {
        enemy.update(&hit_wall);
        enemy.draw();
    }
}

// Possible optimization: use bullet pool instead of creating & deleting new bullets every time
async fn shoot_bullet(bullets: &mut Vec<Bullet>, player: &mut Player, last_shot: &mut f64, score: &mut i32) {
    let current_time = get_time();
    if is_key_down(KeyCode::Space) && current_time - *last_shot > 0.3 {
        let bullet_position = vec2(
            player.position.x + (player.texture.width() * player.scale) / 2.0,
            player.position.y
        );
        bullets.push(Bullet::new(bullet_position));
        *last_shot = current_time;
        *score -= 1;
    }
}

async fn check_collision(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, score: &mut i32) {
    for bullet in bullets.iter_mut() {
        for enemy in enemies.iter_mut() {
            if bullet.position.x < enemy.position.x + enemy.texture.width() * enemy.scale &&   // Left
               bullet.position.x > enemy.position.x &&                                         // Right
               bullet.position.y < enemy.position.y + enemy.texture.height() * enemy.scale &&  // Top
               bullet.position.y > enemy.position.y {                                          // Bottom
                    bullet.collided = true;
                    enemy.collided = true;
                    *score += 10;
            }
        }
    }
    bullets.retain(|bullet| !bullet.collided);
    enemies.retain(|enemy| !enemy.collided);
}

async fn check_round_finished(player: &Player, enemies: &Vec<Enemy>, game_state: &mut GameState) {
    if enemies.is_empty() {
        *game_state = GameState::LevelComplete;
    }
    
    for enemy in enemies {
        if enemy.position.y + enemy.texture.height() * enemy.scale >= player.position.y {
            *game_state = GameState::GameOver;
        }
    }
}

#[macroquad::main("CodeInvaders")]
async fn main() {
    let player_texture = load_texture("assets/rust.png").await.unwrap();

    let mut game_state = GameState::Menu;
    let mut level: i8 = 0;
    let mut score: i32 = 0;
    let mut player = Player::new(player_texture);
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut last_shot = get_time();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Menu => {
                draw_menu(vec![
                    MenuText { text: "CodeInvaders".to_string(), font_size: 50 },
                    MenuText { text: "Press ENTER to Start".to_string(), font_size: 30 },
                ]).await;

                if is_key_pressed(KeyCode::Enter) {
                    init_game(&mut level, &mut player, &mut enemies, &mut bullets, &mut game_state).await;
                }
            }
            GameState::Playing => {
                player.update();
                player.draw();

                calculate_enemy_movement(&mut enemies).await;

                shoot_bullet(&mut bullets, &mut player, &mut last_shot, &mut score).await;

                for bullet in bullets.iter_mut() {
                    bullet.update();
                    bullet.draw();
                }
                bullets.retain(|bullet| bullet.position.y < screen_height());

                check_collision(&mut bullets, &mut enemies, &mut score).await;

                check_round_finished(&player, &enemies, &mut game_state).await;
            }
            GameState::LevelComplete => {
                draw_menu(vec![
                    MenuText { text: "LEVEL COMPLETE".to_string(), font_size: 50 },
                    MenuText { text: format!("SCORE: {score}"), font_size: 50 },
                    MenuText { text: "Press ENTER to Continue".to_string(), font_size: 30 },
                    MenuText { text: "Press ESC to Finish".to_string(), font_size: 30 },
                ]).await;

                if is_key_pressed(KeyCode::Enter) {
                    init_game(&mut level, &mut player, &mut enemies, &mut bullets, &mut game_state).await;
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    level = 0;
                    game_state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                draw_menu(vec![
                    MenuText { text: "GAME OVER".to_string(), font_size: 50 },
                    MenuText { text: format!("SCORE: {score}"), font_size: 50 },
                    MenuText { text: "Press ENTER to Continue".to_string(), font_size: 30 },
                    MenuText { text: "Press ESC to Finish".to_string(), font_size: 30 },
                ]).await;

                level = 0;

                if is_key_pressed(KeyCode::Enter) {
                    init_game(&mut level, &mut player, &mut enemies, &mut bullets, &mut game_state).await;
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }
            }
        }

        next_frame().await
    }
}