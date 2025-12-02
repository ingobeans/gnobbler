use macroquad::prelude::*;

use crate::{
    assets::{Assets, World},
    physics::update_physicsbody,
    utils::*,
};

#[derive(Clone, Copy)]
pub enum AnimState {
    Idle,
    Walk,
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
}
impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            camera_pos: pos + vec2(SCREEN_WIDTH / 2.0, 0.0),
            time: 0.0,
            anim_state: AnimState::Idle,
            grounded: true,
            velocity: Vec2::ZERO,
            jump_frames: 0.0,
            facing_left: false,
        }
    }
    pub fn update(&mut self, delta_time: f32, assets: &Assets) {
        self.time += delta_time;

        let input = get_input_axis();
        let friction_mod;
        self.anim_state = AnimState::Idle;
        if input.x != 0.0 {
            self.anim_state = AnimState::Walk;
            friction_mod = 1.0;
            if self.grounded && input.x.is_sign_positive() != self.velocity.x.is_sign_positive() {
                self.velocity.x = 0.0;
            }
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
        (self.pos, self.grounded) =
            update_physicsbody(self.pos, &mut self.velocity, delta_time, &assets.world);
        self.camera_pos = vec2(self.pos.x.max(SCREEN_WIDTH / 2.0), self.pos.y);
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
