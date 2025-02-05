use std::{collections::HashMap, rc::Rc};
use macroquad::prelude::*;
mod player;
use player::Player;
mod enemy;
use enemy::Enemy;
mod bullet;
use bullet::Bullet;
mod high_scores;
use high_scores::HighScores;
mod name_input;
use name_input::NameInput;

enum GameState {
    Menu,
    Playing,
    LevelComplete,
    GameOver,
    HighScores,
    EnterName
}

struct MenuText {
    text: String,
    font_size: u16,
}

const PADDING: f32 = 10.0;
const FONT_SIZE_LARGE: u16 = 50;
const FONT_SIZE_MEDIUM: u16 = 30;

async fn draw_menu(texts: Vec<MenuText>) {
    let mut total_height = 0.0;

    // Calculate total height of all text items including padding
    for menu_text in &texts {
        let text_size = measure_text(&menu_text.text, None, menu_text.font_size, 1.0);
        total_height += text_size.height + PADDING;
    }
    total_height -= PADDING; // Remove the padding after the last item

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
        current_y += text_size.height + PADDING;
    }
}

// Not used anymore, but I leave it here in case I want to use it in the future
// when using this I need to add the assets first, by using something like this:
// include_bytes!("..\\assets\\python.png")
async fn _load_enemy_texture_from_binary(level: &i8, textures: &HashMap<&str, &[u8]>,) -> Texture2D {
    Texture2D::from_file_with_format(
        textures[
            match *level {
                1 => "python",
                2 => "java",
                3 => "dart",
                4 => "cplusplus",
                _ => "c",
            }
        ],
        Some(ImageFormat::Png),
    )
}

async fn load_texture_from_file(filename: &str) -> Texture2D {
    let fileload_result = load_texture(&format!("assets/{}", filename)).await;

    match fileload_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the texture file: {error:?}"),
    }
}

async fn load_enemy_texture(level: &i8) -> Texture2D {
    let filename = match *level {
        1 => "python.png",
        2 => "java.png",
        3 => "dart.png",
        4 => "cplusplus.png",
        _ => "c.png",
    };

    load_texture_from_file(&filename).await
}

async fn spawn_enemies(enemies: &mut Vec<Enemy>, level: &mut i8, texture: Texture2D, rows: i8) {
    let scale = screen_width() / 20.0 / texture.width();
    let enemy_width = texture.width() * scale;
    let enemy_height = texture.height() * scale;
    let spacing_x = enemy_width + PADDING;
    let spacing_y = enemy_height + PADDING;

    let texture_pointer = Rc::new(texture);

    for row in 0..rows {
        for col in 0..10 {
            let x = col as f32 * spacing_x + PADDING + 1.0;
            let y = row as f32 * spacing_y + PADDING;

            enemies.push(Enemy::new(
                Rc::clone(&texture_pointer),
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

    player.reset();

    bullets.clear();

    enemies.clear();

    let current_enemy_texture = load_enemy_texture(level).await;
    spawn_enemies(enemies, level, current_enemy_texture, 5).await;

    *game_state = GameState::Playing;
}

async fn calculate_enemy_movement(enemies: &mut Vec<Enemy>, delta_time: &f32) {
    let screen_width = screen_width();
    let mut hit_wall = false;

    let first_enemy = enemies.first().unwrap(); //We are sure enemies are always present → unwrap is safe
    let enemy_width = first_enemy.texture.width() * first_enemy.scale;

    // ToDo: Sometimes this is bugged and moves all enemies directly to the bottom
    //       I assume the enemies get updated to often and glitch into the padding,
    //       which triggers the direction change multiple times
    for enemy in enemies.iter() {
        if enemy.position.x + enemy_width >= screen_width - PADDING || enemy.position.x <= PADDING {
            hit_wall = true;
            break;
        }
    }

    for enemy in enemies.iter_mut() {
        enemy.update(&hit_wall, delta_time);
        enemy.draw();
    }
}

// ToDo: Possible optimization: use bullet pool instead of creating & deleting new bullets every time
async fn shoot_bullet(bullets: &mut Vec<Bullet>, player: &mut Player, last_shot: &mut f64, delta_time: &f32, score: &mut i32) {
    let bullet_shoot_speed = 50.0;
    let current_time = get_time();
    if is_key_down(KeyCode::Space) && current_time - *last_shot > (bullet_shoot_speed * *delta_time as f64) {
        let bullet_position = vec2(
            player.position.x + (player.texture.width() * player.scale) / 2.0,
            player.position.y
        );
        bullets.push(Bullet::new(bullet_position));
        *last_shot = current_time;
        *score -= 1;
    }
}

fn is_collision(bullet: &Bullet, enemy: &Enemy) -> bool {
    bullet.position.x < enemy.position.x + enemy.texture.width() * enemy.scale &&   // Left
    bullet.position.x > enemy.position.x &&                                         // Right
    bullet.position.y < enemy.position.y + enemy.texture.height() * enemy.scale &&  // Top
    bullet.position.y > enemy.position.y                                            // Bottom
}

async fn check_collision(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, score: &mut i32) {
    for bullet in bullets.iter_mut() {
        for enemy in enemies.iter_mut() {
            if is_collision(bullet, enemy) {                                          // Bottom
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
            *game_state = GameState::EnterName;
        }
    }
}

async fn handle_menu(game_state: &mut GameState, level: &mut i8, player: &mut Player, enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>) {
    draw_menu(vec![
        MenuText { text: "CodeInvaders".to_string(), font_size: FONT_SIZE_LARGE },
        MenuText { text: "Press ENTER to Start".to_string(), font_size: FONT_SIZE_MEDIUM },
        MenuText { text: "Press H to show highscores".to_string(), font_size: FONT_SIZE_MEDIUM },
    ]).await;

    if is_key_pressed(KeyCode::Enter) {
        init_game(level, player, enemies, bullets, game_state).await;
    }
    if is_key_pressed(KeyCode::H) {
        *game_state = GameState::HighScores;
    }
}

async fn handle_playing(game_state: &mut GameState, player: &mut Player, enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>, last_shot: &mut f64, delta_time: &f32, score: &mut i32) {
    player.update(&delta_time);
    player.draw();

    calculate_enemy_movement(enemies, &delta_time).await;

    shoot_bullet(bullets, player, last_shot, &delta_time, score).await;

    for bullet in bullets.iter_mut() {
        bullet.update(&delta_time);
        bullet.draw();
    }
    bullets.retain(|bullet| bullet.position.y < screen_height());

    check_collision(bullets, enemies, score).await;

    check_round_finished(&player, &enemies, game_state).await;

    if is_key_pressed(KeyCode::Escape) {
        *game_state = GameState::EnterName;
    }
}

async fn handle_level_complete(game_state: &mut GameState, level: &mut i8, player: &mut Player, enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>, score: &i32) {
    draw_menu(vec![
        MenuText { text: "LEVEL COMPLETE".to_string(), font_size: FONT_SIZE_LARGE },
        MenuText { text: format!("SCORE: {score}"), font_size: FONT_SIZE_LARGE },
        MenuText { text: "Press ENTER to continue".to_string(), font_size: FONT_SIZE_MEDIUM },
        MenuText { text: "Press ESC to finish".to_string(), font_size: FONT_SIZE_MEDIUM },
    ]).await;

    if is_key_pressed(KeyCode::Enter) {
        init_game(level, player, enemies, bullets, game_state).await;
        return;
    }
    if is_key_pressed(KeyCode::Escape) {
        *level = 0;
        *game_state = GameState::EnterName;
    }
}

async fn handle_game_over(game_state: &mut GameState, score: &mut i32, level: &mut i8) {
    draw_menu(vec![
        MenuText { text: "GAME OVER".to_string(), font_size: FONT_SIZE_LARGE },
        MenuText { text: format!("SCORE: {score}"), font_size: FONT_SIZE_LARGE },
        MenuText { text: "Press ESC to finish".to_string(), font_size: FONT_SIZE_MEDIUM },
        MenuText { text: "Press H to show highscores".to_string(), font_size: FONT_SIZE_MEDIUM },
    ]).await;

    if is_key_pressed(KeyCode::Escape) ||
       is_key_pressed(KeyCode::Enter) {
        *level = 0;
        *score = 0;
        *game_state = GameState::Menu;
        return;
    }
    if is_key_pressed(KeyCode::H) {
        *level = 0;
        *score = 0;
        *game_state = GameState::HighScores;
    }
}

async fn handle_enter_name(game_state: &mut GameState, high_scores: &mut HighScores, name_input: &mut NameInput, score: &i32) {
    if !high_scores.qualifies(*score) {
        *game_state = GameState::GameOver;
        return;
    }

    name_input.update();
    name_input.draw();

    if is_key_pressed(KeyCode::Enter) {
        high_scores.add_score(name_input.name.clone(), *score);
        *game_state = GameState::GameOver;
    }
}

async fn handle_high_scores(game_state: &mut GameState, high_scores: &HighScores) {
    let mut scores = high_scores.display().iter().map(
        |entry| MenuText { text: entry.clone(), font_size: FONT_SIZE_MEDIUM }
    ).collect::<Vec<MenuText>>();

    let mut menu_texts = vec![
        MenuText { text: "HIGH SCORES".to_string(), font_size: FONT_SIZE_LARGE },
        MenuText { text: "Press ESC to exit".to_string(), font_size: FONT_SIZE_MEDIUM },
        MenuText { text: " ".to_string(), font_size: FONT_SIZE_MEDIUM },
    ];

    menu_texts.append(&mut scores);

    draw_menu(menu_texts).await;

    if is_key_pressed(KeyCode::Escape) {
        *game_state = GameState::Menu;
    }
}

#[macroquad::main("CodeInvaders")]
async fn main() {
    // Initiate globaly needed game assets
    let mut delta_time;
    let mut game_state = GameState::Menu;
    let mut level: i8 = 0;
    let mut score: i32 = 0;
    let mut high_scores = HighScores::new();
    high_scores.load().unwrap_or_default(); // right now i implemented this to always return true, so no use of error handling
    let mut name_input = NameInput::new();
    let mut player = Player::new(load_texture_from_file("rust.png").await);
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut last_shot = get_time();

    // Run game
    clear_background(BLACK);
    loop {
        delta_time = get_frame_time();

        match game_state {
            // ToDo: This looks pretty bad. Find a better way to handle globaly needed stuff like the enemy list
            GameState::Menu => handle_menu(&mut game_state, &mut level, &mut player, &mut enemies, &mut bullets).await,
            GameState::Playing => handle_playing(&mut game_state, &mut player, &mut enemies, &mut bullets, &mut last_shot, &delta_time, &mut score).await,
            GameState::LevelComplete => handle_level_complete(&mut game_state, &mut level, &mut player, &mut enemies, &mut bullets, &score).await,
            GameState::GameOver => handle_game_over(&mut game_state, &mut score, &mut level).await,
            GameState::EnterName => handle_enter_name(&mut game_state, &mut high_scores, &mut name_input, &score).await,
            GameState::HighScores => handle_high_scores(&mut game_state, &high_scores).await,
            }

        next_frame().await
    }
}