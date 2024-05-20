#![allow(unused_variables, dead_code, unused_imports)]

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    // high_score: u32,
    current_score: u32,
    ferris_index: i32,
    // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // high_score: 0,
            ferris_index: 0,
            current_score: 0,
            // spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}
fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);

    player.translation = Vec2::new(200.0, 200.0);
    player.rotation = std::f32::consts::FRAC_PI_2; // "UP"
    player.scale = 1.0;
    player.collision = true;

    let ferris = game.add_sprite("ferris1", "ferris.png");
    ferris.translation = Vec2::new(100.0, 100.0);
    ferris.collision = true;
    ferris.scale = 0.1;
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Collisions
    // engine.show_colliders = true; // requires valid .collider file -> run collider <path> after installing --example collider
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.current_score += 1;
            println!("Current score: {}", game_state.current_score);
        }
    }

    let player = engine.sprites.get_mut("player").unwrap();

    // Handle Movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // mouse events
}
