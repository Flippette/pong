use crate::*;

use color_eyre::eyre::Result;
use macroquad::prelude::*;

pub struct Player {
    pub id: usize,
    pub score: usize,
    pub movement_keys: (KeyCode, KeyCode),
    pub paddle: Paddle,
}

pub struct Paddle {
    pub(crate) transform: Transform,
    pub sprite: Texture2D,
}

impl Player {
    pub async fn create_from_file(
        path: &str,
        id: usize,
        key_up: KeyCode,
        key_down: KeyCode,
        x_pos: f32,
    ) -> Result<Self> {
        let sprite = load_texture(path).await?;

        Ok(Self {
            id,
            score: 0,
            movement_keys: (key_up, key_down),
            paddle: Paddle {
                transform: Transform {
                    position: Vec2::new(
                        x_pos,
                        screen_height() / 2.0 + sprite_center_offset(&sprite).y,
                    ),
                    velocity: Vec2::ZERO,
                },
                sprite,
            },
        })
    }

    pub fn process_input(&mut self) {
        self.paddle.transform.velocity = match (
            is_key_down(self.movement_keys.0),
            is_key_down(self.movement_keys.1),
        ) {
            (true, false) => -Vec2::Y,
            (false, true) => Vec2::Y,
            _ => Vec2::ZERO,
        };

        match is_out_of_bounds(self.paddle.transform.position, &self.paddle.sprite) {
            OutOfBoundsType::Top => {
                self.paddle.transform.position.y = 0.1;
                self.paddle.transform.velocity.y = 0.0;
            }
            OutOfBoundsType::Bottom => {
                self.paddle.transform.position.y =
                    screen_height() - self.paddle.sprite.height() - 0.1;
                self.paddle.transform.velocity.y = 0.0;
            }
            _ => (),
        }
    }

    pub fn tick(&mut self) {
        self.paddle.transform.position +=
            self.paddle.transform.velocity * PADDLE_SPEED * get_frame_time();
    }
}
