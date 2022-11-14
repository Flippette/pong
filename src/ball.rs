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
                position: screen_center() - sprite_center_offset(&sprite),
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
                CollideType::Horizontal => {
                    self.transform.velocity = -self.transform.velocity;

                    // players can influence the ball's velocity
                    if player.paddle.transform.velocity.y != 0.0 {
                        self.transform.velocity.y *= player.paddle.transform.velocity.y;
                    }
                    break; // the ball will only ever collide with 1 player at a time
                }
                CollideType::Vertical => {
                    self.transform.velocity.y = -self.transform.velocity.y;

                    // avoid infinite flip loop
                    if self.transform.position.y + self.sprite.height()
                        == player.paddle.transform.position.y
                    {
                        self.transform.position.y -= 0.1;
                    } else if self.transform.position.y
                        == player.paddle.transform.position.y + player.paddle.sprite.height()
                    {
                        self.transform.position.y += 0.1;
                    }

                    break; // the ball will only ever collide with 1 player at a time
                }
            }
        }
    }

    pub fn check_out_of_bounds(&mut self, player1_score: &mut usize, player2_score: &mut usize) {
        match is_out_of_bounds(self.transform.position, &self.sprite) {
            OutOfBoundsType::Left => {
                *player1_score += 1;

                // "respawn" ball
                // TODO: make this into a function
                self.transform.position = screen_center() - sprite_center_offset(&self.sprite);
                self.transform.velocity = rand_unit_vec() * BALL_SPEED;
            }
            OutOfBoundsType::Right => {
                *player2_score += 1;

                // "respawn" ball
                // TODO: make this into a function
                self.transform.position = screen_center() - sprite_center_offset(&self.sprite);
                self.transform.velocity = rand_unit_vec() * BALL_SPEED;
            }
            OutOfBoundsType::Top | OutOfBoundsType::Bottom => {
                // simply bounce the ball
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
