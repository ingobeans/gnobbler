use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{
    assets::Assets,
    player::Player,
    utils::{SCREEN_HEIGHT, SCREEN_WIDTH, create_camera},
};

mod assets;
mod physics;
mod player;
mod utils;

struct Gnobbler<'a> {
    assets: &'a Assets,
    player: Player,
    camera: Camera2D,
}
impl<'a> Gnobbler<'a> {
    fn new(assets: &'a Assets) -> Self {
        Self {
            player: Player::new(assets.world.get_player_spawn()),
            camera: create_camera(SCREEN_WIDTH, SCREEN_HEIGHT),
            assets,
        }
    }
    fn update(&mut self) {
        // cap delta time to a minimum of 60 fps.
        let delta_time = get_frame_time().min(1.0 / 60.0);
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);

        self.player.update(delta_time, self.assets);
        self.camera.target = self.player.camera_pos.floor();
        set_camera(&self.camera);
        clear_background(Color::from_hex(0x00aaff));
        for chunk in &self.assets.world.collision {
            chunk.draw(self.assets);
        }
        for chunk in &self.assets.world.details {
            chunk.draw(self.assets);
        }
        for chunk in &self.assets.world.one_way_collision {
            chunk.draw(self.assets);
        }

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
