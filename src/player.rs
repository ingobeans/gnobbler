use macroquad::prelude::*;

use crate::{
    assets::{Assets, WorldState},
    physics::update_physicsbody,
    utils::*,
};

#[derive(Clone, Copy)]
pub enum AnimState {
    Idle,
    Walk,
    Die,
}

pub enum PlayerState {
    Active,
    Died,
}

pub enum PlayerUpdateResult {
    None,
    RestartLevel,
}

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,

    pub velocity: Vec2,
    pub grounded: bool,
    pub jump_frames: f32,

    pub time: f32,
    pub anim_state: AnimState,
    pub facing_left: bool,

    pub player_state: PlayerState,
}
impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            camera_pos: pos + vec2(SCREEN_WIDTH / 2.0, -24.0),
            time: 0.0,
            anim_state: AnimState::Idle,
            grounded: true,
            velocity: Vec2::ZERO,
            jump_frames: 0.0,
            facing_left: false,
            player_state: PlayerState::Active,
        }
    }
    pub fn alive(&self) -> bool {
        matches!(self.player_state, PlayerState::Active)
    }
    pub fn die(&mut self) {
        assert!(matches!(self.player_state, PlayerState::Active));
        self.player_state = PlayerState::Died;
        self.time = 0.0;
    }
    pub fn update(
        &mut self,
        delta_time: f32,
        assets: &Assets,
        world_state: &mut WorldState,
        current_level: usize,
    ) -> PlayerUpdateResult {
        self.time += delta_time;
        match self.player_state {
            PlayerState::Active => {
                let input = get_input_axis();
                let friction_mod;
                self.anim_state = AnimState::Idle;
                if input.x != 0.0 {
                    self.anim_state = AnimState::Walk;
                    friction_mod = 1.0;
                    self.facing_left = input.x.is_sign_negative();
                    self.velocity.x += input.x * ACCELERATION * delta_time;
                } else {
                    friction_mod = 2.5;
                }

                if self.grounded {
                    self.jump_frames = 0.0;
                }
                if is_key_down(KeyCode::Space)
                    && (self.grounded || (self.jump_frames > 0.0 && self.jump_frames < 0.5))
                {
                    if self.jump_frames == 0.0 && is_key_pressed(KeyCode::Space) {
                        self.velocity.y -= 2.3 * 60.0;
                    } else {
                        self.velocity.y -= 30.0 * 10.0 * delta_time;
                    }
                    self.jump_frames += delta_time;
                }

                self.velocity.x -= self.velocity.x
                    * if self.grounded {
                        GROUND_FRICTION * friction_mod
                    } else {
                        AIR_DRAG * friction_mod
                    }
                    * delta_time;

                self.velocity.y += GRAVITY * delta_time;
                let touched_death_tile;
                let broke_block;
                (self.pos, self.grounded, touched_death_tile, broke_block) = update_physicsbody(
                    self.pos,
                    &mut self.velocity,
                    delta_time,
                    &assets.levels[current_level],
                    &world_state.broken_tiles,
                );
                if touched_death_tile {
                    self.die();
                }
                if let Some(broke_block) = broke_block {
                    world_state.broken_tiles.push(broke_block);
                }

                self.camera_pos.x = self.pos.x.max(SCREEN_WIDTH / 2.0);
                let target = self.pos.y - 24.0;
                if self.camera_pos.y < target {
                    self.camera_pos.y = target;
                } else {
                    let delta = self.camera_pos.y - target;
                    let max_delta = 3.5 * 8.0;
                    if delta.abs() > max_delta {
                        self.camera_pos.y =
                            max_delta * if delta < 0.0 { -1.0 } else { 1.0 } + target;
                    }
                }
                PlayerUpdateResult::None
            }
            PlayerState::Died => {
                self.velocity = self.velocity.lerp(vec2(0.0, -32.0), delta_time * 8.0);
                self.pos += self.velocity * delta_time;
                self.anim_state = AnimState::Die;
                if self.time * 1000.0
                    >= assets.player.animations[self.anim_state as usize].total_length as f32
                {
                    // temporary, just to hide player off screen
                    PlayerUpdateResult::RestartLevel
                } else {
                    PlayerUpdateResult::None
                }
            }
        }
    }
    pub fn draw(&mut self, assets: &Assets) {
        draw_texture_ex(
            assets.player.animations[self.anim_state as usize]
                .get_at_time((self.time * 1000.0) as u32),
            self.pos.x.floor(),
            self.pos.y.floor(),
            WHITE,
            DrawTextureParams {
                flip_x: self.facing_left,
                ..Default::default()
            },
        );
    }
}
