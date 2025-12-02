use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{
    assets::*,
    player::Player,
    utils::{SCREEN_HEIGHT, SCREEN_WIDTH, create_camera},
};

mod assets;
mod enemy;
mod physics;
mod player;
mod utils;

struct Gnobbler<'a> {
    assets: &'a Assets,
    player: Player,
    camera: Camera2D,
    world_state: WorldState,
}
impl<'a> Gnobbler<'a> {
    fn new(assets: &'a Assets) -> Self {
        Self {
            player: Player::new(assets.world.get_player_spawn()),
            camera: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            world_state: assets.world.world_state.clone(),
            assets,
        }
    }
    fn draw_world(&self) {
        for layer in [
            &self.assets.world.collision,
            &self.assets.world.background,
            &self.assets.world.death,
            &self.assets.world.details,
            &self.assets.world.one_way_collision,
        ] {
            for ((cx, cy), chunk) in layer.iter() {
                for (index, tile) in chunk.tiles.iter().enumerate() {
                    if *tile == 0 {
                        continue;
                    }
                    let tile = *tile - 1;
                    let x = index % 16;
                    let y = index / 16;
                    if tile == 48
                        && self
                            .world_state
                            .broken_tiles
                            .contains(&(*cx + x as i16, *cy + y as i16))
                    {
                        continue;
                    }
                    self.assets.tileset.draw_tile(
                        *cx as f32 * 8.0 + (x * 8) as f32,
                        *cy as f32 * 8.0 + (y * 8) as f32,
                        (tile % 16) as f32,
                        (tile / 16) as f32,
                        None,
                    );
                }
            }
        }
    }
    fn update(&mut self) {
        // cap delta time to a minimum of 60 fps.
        let delta_time = get_frame_time().min(1.0 / 60.0);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);

        self.player
            .update(delta_time, self.assets, &mut self.world_state);
        self.camera.target = self.player.camera_pos.floor();
        set_camera(&self.camera);
        clear_background(Color::from_hex(0x00aaff));
        self.draw_world();
        let mut enemies = Vec::new();
        std::mem::swap(&mut enemies, &mut self.world_state.enemies);
        let mut player_squashed_enemy = false;
        enemies.retain_mut(|enemy| {
            enemy.update(delta_time, self.assets, &self.world_state);
            enemy.draw(self.assets);
            if !player_squashed_enemy
                && self.player.alive()
                && self.player.pos.distance_squared(enemy.pos) < 64.0
            {
                player_squashed_enemy = true;
                if self.player.pos.y >= enemy.pos.y || self.player.velocity.y < 0.0 {
                    self.player.die();
                    true
                } else {
                    self.player.velocity.y = -2.5 * 60.0;
                    false
                }
            } else {
                true
            }
        });
        std::mem::swap(&mut enemies, &mut self.world_state.enemies);

        self.player.draw(self.assets);

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &self.camera.render_target.as_ref().unwrap().texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    SCREEN_WIDTH * scale_factor,
                    SCREEN_HEIGHT * scale_factor,
                )),
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("gnobbler")]
async fn main() {
    let assets = Assets::load();
    let mut gnobbler = Gnobbler::new(&assets);
    loop {
        gnobbler.update();
        next_frame().await;
    }
}
