mod ball;
mod player;

use color_eyre::eyre::Result;
use macroquad::prelude::*;

use ball::Ball;
use player::Player;

const BALL_SPRITE_PATH: &str = "ball.png";
const PADDLE_SPRITE_PATH: &str = "paddle.png";

const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 500.0;
const BALL_ACCEL_RATE: f32 = 0.1;

const SCORE_FONT_SIZE: f32 = 72.0;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    color_eyre::install()?;

    set_pc_assets_folder("assets");

    let mut ball = Ball::create_from_file(BALL_SPRITE_PATH).await?;

    let mut player1 = Player::create_from_file(
        PADDLE_SPRITE_PATH,
        0,
        KeyCode::Up,
        KeyCode::Down,
        screen_width() / 6.0 * 5.0,
    )
    .await?;
    let mut player2 = Player::create_from_file(
        PADDLE_SPRITE_PATH,
        0,
        KeyCode::W,
        KeyCode::S,
        screen_width() / 6.0,
    )
    .await?;

    loop {
        clear_background(BLACK);

        player1.process_input();
        player1.tick(); // movement

        player2.process_input();
        player2.tick(); // movement

        ball.check_player_collision(&[&player1, &player2]);
        ball.check_out_of_bounds(&mut player1.score, &mut player2.score);
        ball.tick(); // movement

        draw_texture(
            ball.sprite,
            ball.transform.position.x,
            ball.transform.position.y,
            WHITE,
        );

        draw_texture(
            player1.paddle.sprite,
            player1.paddle.transform.position.x,
            player1.paddle.transform.position.y,
            WHITE,
        );

        draw_text(
            &player1.score.to_string(),
            screen_width() / 4.0 * 3.0,
            SCORE_FONT_SIZE / 2.0 + 20.0,
            SCORE_FONT_SIZE,
            WHITE,
        );

        draw_texture(
            player2.paddle.sprite,
            player2.paddle.transform.position.x,
            player2.paddle.transform.position.y,
            WHITE,
        );

        draw_text(
            &player2.score.to_string(),
            screen_width() / 4.0,
            SCORE_FONT_SIZE / 2.0 + 20.0,
            SCORE_FONT_SIZE,
            WHITE,
        );

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "pong".to_string(),
        window_width: 800,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}

fn screen_center() -> Vec2 {
    Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
}

// coordinates start from top left to bottom right;
// this is to convert object coordinates to sprite center coorinates
fn sprite_center_offset(texture: &Texture2D) -> Vec2 {
    Vec2::new(texture.width() / 2.0, texture.height() / 2.0)
}

fn is_out_of_bounds(position: Vec2, texture: &Texture2D) -> OutOfBoundsType {
    if position.x < 0.0 {
        OutOfBoundsType::Left
    } else if position.x + texture.width() > screen_width() {
        OutOfBoundsType::Right
    } else if position.y < 0.0 {
        OutOfBoundsType::Top
    } else if position.y + texture.height() > screen_height() {
        OutOfBoundsType::Bottom
    } else {
        OutOfBoundsType::InBounds
    }
}

fn rand_unit_vec() -> Vec2 {
    Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize()
}

fn collide(a: Vec2, a_texture: &Texture2D, b: Vec2, b_texture: &Texture2D) -> CollideType {
    let a = a + sprite_center_offset(a_texture);
    let b = b + sprite_center_offset(b_texture);

    let diff_vec = (b - a).abs();

    // since we're comparing 2 uniform axis rectangles, simple x and y distance checks are okay
    if diff_vec.x < a_texture.width() / 2.0 + b_texture.width() / 2.0 {
        if a.y + a_texture.height() == b.y || b.y + b_texture.height() == a.y {
            return CollideType::Vertical;
        } else if diff_vec.y < a_texture.height() / 2.0 + b_texture.height() / 2.0 {
            return CollideType::Horizontal;
        }
    }

    CollideType::Separate
}

enum OutOfBoundsType {
    Left,
    Right,
    Top,
    Bottom,
    InBounds,
}

enum CollideType {
    Horizontal,
    Vertical,
    Separate,
}

struct Transform {
    pub position: Vec2,
    pub velocity: Vec2,
}
