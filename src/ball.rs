use crate::*;

use color_eyre::eyre::Result;
use macroquad::prelude::*;

pub struct Ball {
    pub(crate) transform: Transform,
    pub sprite: Texture2D,
}

impl Ball {
    pub async fn create_from_file(path: &str) -> Result<Self> {
        let sprite = load_texture(path).await?;

        Ok(Self {
            transform: Transform {
                position: screen_center() + sprite_center_offset(&sprite),
                velocity: rand_unit_vec() * BALL_SPEED,
            },
            sprite,
        })
    }

    pub fn check_player_collision(&mut self, players: &[&Player]) {
        for player in players {
            match collide(
                self.transform.position,
                &self.sprite,
                player.paddle.transform.position,
                &player.paddle.sprite,
            ) {
                CollideType::Separate => continue,
                CollideType::Horizontal => self.transform.velocity = -self.transform.velocity,
                CollideType::Vertical => self.transform.velocity.y = -self.transform.velocity.y,
            }
        }
    }

    pub fn check_out_of_bounds(&mut self, player1_score: &mut usize, player2_score: &mut usize) {
        match is_out_of_bounds(self.transform.position, &self.sprite) {
            OutOfBoundsType::Left => {
                *player1_score += 1;
                self.transform.position = screen_center() + sprite_center_offset(&self.sprite);
                self.transform.velocity = rand_unit_vec() * BALL_SPEED;
            }
            OutOfBoundsType::Right => {
                *player2_score += 1;
                self.transform.position = screen_center() + sprite_center_offset(&self.sprite);
                self.transform.velocity = rand_unit_vec() * BALL_SPEED;
            }
            OutOfBoundsType::Top | OutOfBoundsType::Bottom => {
                self.transform.velocity.y = -self.transform.velocity.y;
            }
            OutOfBoundsType::InBounds => (),
        };
    }

    pub fn tick(&mut self) {
        self.transform.position += self.transform.velocity * get_frame_time();
        self.transform.velocity += self.transform.velocity * BALL_ACCEL_RATE * get_frame_time();
    }
}
