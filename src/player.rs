use macroquad::prelude::*;

use crate::{assets::Assets, utils::SCREEN_WIDTH};

#[derive(Clone, Copy)]
pub enum AnimState {
    Idle,
    Walk,
}

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,
    pub time: f32,
    pub anim_state: AnimState,
}
impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            camera_pos: pos + vec2(SCREEN_WIDTH / 2.0, 0.0),
            time: 0.0,
            anim_state: AnimState::Idle,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        self.camera_pos = vec2(self.pos.x.max(SCREEN_WIDTH / 2.0), self.pos.y);
        self.time += delta_time;
    }
    pub fn draw(&mut self, assets: &Assets) {
        draw_texture(
            assets.player.animations[self.anim_state as usize]
                .get_at_time((self.time * 1000.0) as u32),
            self.pos.x,
            self.pos.y,
            WHITE,
        );
    }
}
