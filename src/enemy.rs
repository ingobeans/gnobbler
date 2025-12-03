use macroquad::prelude::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::{assets::Assets, physics::update_physicsbody};

#[derive(FromPrimitive, ToPrimitive, Clone)]
pub enum EnemyType {
    Snail,
}
impl EnemyType {
    fn speed(&self) -> f32 {
        match self {
            EnemyType::Snail => 8.0,
        }
    }
}

#[derive(Clone)]
pub struct Enemy {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub ty: EnemyType,
    pub facing_left: bool,
    pub time: f32,
}
impl Enemy {
    pub fn new(pos: Vec2, ty: EnemyType) -> Self {
        Self {
            pos,
            ty,
            facing_left: true,
            time: 0.0,
            velocity: Vec2::ZERO,
        }
    }
    pub fn update(&mut self, delta_time: f32, assets: &Assets, broken_tiles: &[(i16, i16)]) {
        self.time += delta_time;
        self.velocity.x = if self.facing_left { -1.0 } else { 1.0 } * self.ty.speed();
        (self.pos, _, _, _) = update_physicsbody(
            self.pos,
            &mut self.velocity,
            delta_time,
            &assets.world,
            broken_tiles,
        );
    }
    pub fn draw(&self, assets: &Assets) {
        let id = self.ty.to_usize().unwrap();
        draw_texture_ex(
            assets.enemies.animations[id].get_at_time((self.time * 1000.0) as u32),
            self.pos.x.floor() - 4.0,
            self.pos.y.floor() - 8.0,
            WHITE,
            DrawTextureParams {
                flip_x: self.facing_left,
                ..Default::default()
            },
        );
    }
}
