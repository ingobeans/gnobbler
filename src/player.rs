use macroquad::prelude::*;

use crate::{assets::Assets, utils::SCREEN_WIDTH};

pub struct Player {
    pub pos: Vec2,
    pub camera_pos: Vec2,
    pub time: f32,
}
impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            camera_pos: pos + vec2(SCREEN_WIDTH / 2.0, 0.0),
            time: 0.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        self.camera_pos = vec2(self.pos.x.max(SCREEN_WIDTH / 2.0), self.pos.y);
    }
    pub fn draw(&mut self, assets: &Assets) {}
}
