use std::f32::consts::E;

use macroquad::{
    audio::{PlaySoundParams, play_sound, set_sound_volume},
    miniquad::window::screen_size,
    prelude::*,
};

use crate::{
    assets::*,
    player::{Player, PlayerUpdateResult},
    utils::*,
};

mod assets;
mod enemy;
mod physics;
mod player;
mod utils;

struct Gnobbler<'a> {
    in_main_menu: bool,
    assets: &'a Assets,
    player: Player,
    camera: Camera2D,
    world_state: WorldState,
    time: f32,
    current_level: usize,
    volume: f32,
    actual_volume: f32,
    coins: u32,
}
impl<'a> Gnobbler<'a> {
    fn new(assets: &'a Assets, default_volume: f32) -> Self {
        let (world_state, mut player) = assets.levels[0].load_level();
        player.pos = vec2(-32.0, 0.0);
        let mut camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
        camera.target = vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);

        let mut new = Self {
            coins: 0,
            in_main_menu: true,
            player,
            world_state,
            camera,
            assets,
            time: 0.0,
            current_level: 0,
            volume: 0.0,
            actual_volume: 0.0,
        };
        new.set_volume(default_volume);
        new
    }
    fn set_volume(&mut self, new: f32) {
        let actual = new.powf(E);
        set_sound_volume(&self.assets.song, actual);
        self.actual_volume = actual;
        self.volume = new;
    }
    fn draw_world(&self) {
        for layer in [
            &self.assets.levels[self.current_level].background,
            &self.assets.levels[self.current_level].collision,
            &self.assets.levels[self.current_level].details,
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
        self.time += delta_time;
        let (actual_screen_width, actual_screen_height) = screen_size();
        let scale_factor =
            (actual_screen_width / SCREEN_WIDTH).min(actual_screen_height / SCREEN_HEIGHT);

        let result = self.player.update(
            delta_time,
            self.assets,
            &mut self.world_state,
            self.current_level,
        );

        match result {
            PlayerUpdateResult::RestartLevel => {
                (self.world_state, self.player) =
                    self.assets.levels[self.current_level].load_level();
            }
            PlayerUpdateResult::PlayStompSfx => {
                play_sound(
                    &self.assets.stomp_sfx,
                    PlaySoundParams {
                        looped: false,
                        volume: self.actual_volume,
                    },
                );
            }
            PlayerUpdateResult::PlayTrampolineSfx => {
                play_sound(
                    &self.assets.jump_sfx,
                    PlaySoundParams {
                        looped: false,
                        volume: self.actual_volume,
                    },
                );
            }
            PlayerUpdateResult::NextLevel => {
                self.current_level += 1;
                (self.world_state, self.player) =
                    self.assets.levels[self.current_level].load_level();
            }
            PlayerUpdateResult::None => {}
        }

        if !self.in_main_menu {
            self.camera.target = self.player.camera_pos.floor();
        }
        set_camera(&self.camera);
        clear_background(Color::from_hex(0x00aaff));
        self.draw_world();
        let pos = self.assets.levels[self.current_level].finish_pos;
        let mut pos = vec2((pos.0 * 8) as f32 + 8.0, (pos.1 * 8) as f32 - 32.0);
        if self.world_state.boat_offset > BOAT_WAIT_TIME {
            pos.x += (self.world_state.boat_offset - BOAT_WAIT_TIME) * BOAT_MOVE_SPEED;
        }
        draw_texture(&self.assets.boat, pos.x, pos.y, WHITE);
        let mut player_squashed_enemy = false;
        self.world_state.enemies.retain_mut(|enemy| {
            if !enemy.loaded
                && (0.0..self.player.camera_pos.x + SCREEN_WIDTH / 2.0).contains(&enemy.pos.x)
            {
                enemy.loaded = true;
            }
            if enemy.loaded {
                enemy.update(
                    delta_time,
                    self.assets,
                    &self.world_state.broken_tiles,
                    self.current_level,
                );
                enemy.draw(self.assets);
                if !player_squashed_enemy
                    && self.player.alive()
                    && self.player.pos.distance_squared(enemy.pos) < 64.0
                {
                    player_squashed_enemy = true;
                    if self.player.pos.y >= enemy.pos.y || self.player.velocity.y < 0.0 {
                        play_sound(
                            &self.assets.stomp_sfx,
                            PlaySoundParams {
                                looped: false,
                                volume: self.actual_volume,
                            },
                        );
                        self.player.die();
                        true
                    } else {
                        play_sound(
                            &self.assets.stomp_sfx,
                            PlaySoundParams {
                                looped: false,
                                volume: self.actual_volume,
                            },
                        );
                        self.player.velocity.y = -2.5 * 60.0;
                        false
                    }
                } else {
                    true
                }
            } else {
                true
            }
        });
        self.world_state.coins.retain(|(x, y)| {
            let pos = vec2(*x as f32 * 8.0, *y as f32 * 8.0);
            draw_texture(
                self.assets.coin.get_at_time((self.time * 1000.0) as u32),
                pos.x,
                pos.y,
                WHITE,
            );
            if self.player.pos.distance_squared(pos) < 64.0 {
                self.coins += 1;
                play_sound(
                    &self.assets.coin_sfx,
                    PlaySoundParams {
                        looped: false,
                        volume: self.actual_volume,
                    },
                );
                false
            } else {
                true
            }
        });

        self.player.draw(self.assets);

        if !self.in_main_menu {
            draw_texture(
                &self.assets.bar_ui,
                self.camera.target.x - SCREEN_WIDTH / 2.0,
                self.camera.target.y - SCREEN_HEIGHT / 2.0 + 0.0,
                WHITE,
            );
            self.assets.draw_number(
                &self.coins.to_string(),
                self.camera.target.x - SCREEN_WIDTH / 2.0 + 9.0,
                self.camera.target.y - SCREEN_HEIGHT / 2.0 + 1.0,
            );
            self.assets.draw_number(
                &format!("{:0>2}", (self.time / 60.0) as u32),
                self.camera.target.x - SCREEN_WIDTH / 2.0 + 32.0,
                self.camera.target.y - SCREEN_HEIGHT / 2.0 + 1.0,
            );
            self.assets.draw_number(
                &format!("{:0>2}", (self.time % 60.0) as u32),
                self.camera.target.x - SCREEN_WIDTH / 2.0 + 32.0 + 14.0,
                self.camera.target.y - SCREEN_HEIGHT / 2.0 + 1.0,
            );
        }
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
        if self.in_main_menu {
            let menu_size = self.assets.menu_body.size();
            let menu_pos = vec2(
                (actual_screen_width - menu_size.x * scale_factor) / 2.0,
                11.0 * scale_factor,
            );
            draw_texture_ex(
                &self.assets.menu_body,
                menu_pos.x,
                menu_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(menu_size * scale_factor),
                    ..Default::default()
                },
            );
            let start_btn = UIImageButton::new(
                menu_pos + vec2(7.0 * scale_factor, 29.0 * scale_factor),
                &self.assets.start_btn.frames[0].0,
                &self.assets.start_btn.frames[1].0,
                scale_factor,
            );
            if start_btn.is_hovered() && is_mouse_button_pressed(MouseButton::Left) {
                self.load_next_level();
            }
            for (m, offset, anim) in [
                (-1.0, 0.0, &self.assets.minus_btn),
                (1.0, 84.0, &self.assets.plus_btn),
            ] {
                let btn = UIImageButton::new(
                    menu_pos + vec2((7.0 + offset) * scale_factor, 59.0 * scale_factor),
                    &anim.frames[0].0,
                    &anim.frames[1].0,
                    scale_factor,
                );
                btn.draw();
                if btn.is_hovered() && is_mouse_button_pressed(MouseButton::Left) {
                    self.set_volume((self.volume + m * 0.1).clamp(0.0, 2.0));
                }
            }
            let volume_bar_pos = menu_pos + vec2(17.0 * scale_factor, 59.0 * scale_factor);
            let volume_bar_size = vec2(73.0, 9.0);
            draw_rectangle(
                volume_bar_pos.x,
                volume_bar_pos.y,
                volume_bar_size.x * scale_factor * self.volume / 2.0,
                volume_bar_size.y * scale_factor,
                Color::from_hex(0x8e5252),
            );
            start_btn.draw();
        }
    }
    fn load_next_level(&mut self) {
        if self.current_level == 0 {
            self.time = 0.0;
        }
        self.current_level += 1;
        (self.world_state, self.player) = self.assets.levels[self.current_level].load_level();
        self.in_main_menu = false;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "gnobbler".to_string(),
        window_width: SCREEN_WIDTH as i32 * 3,
        window_height: SCREEN_HEIGHT as i32 * 3,

        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let assets = Assets::load().await;

    #[cfg(debug_assertions)]
    let default_volume = 0.0;
    #[cfg(not(debug_assertions))]
    let default_volume = 1.0;
    play_sound(
        &assets.song,
        PlaySoundParams {
            looped: true,
            volume: default_volume,
        },
    );
    let mut gnobbler = Gnobbler::new(&assets, default_volume);

    #[cfg(debug_assertions)]
    {
        use std::env::args;

        if let Some(index) = args().find_map(|f| {
            f.strip_prefix("level=")
                .and_then(|f| f.parse::<usize>().ok())
        }) {
            gnobbler.current_level = index;
            (gnobbler.world_state, gnobbler.player) = assets.levels[index].load_level();
            gnobbler.in_main_menu = gnobbler.current_level == 0;
        }
    }
    loop {
        gnobbler.update();
        next_frame().await;
    }
}
