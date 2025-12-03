use std::collections::HashMap;

use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::{
    audio::{Sound, load_sound_from_bytes},
    prelude::*,
};
use num_traits::FromPrimitive;

use crate::{
    enemy::{Enemy, EnemyType},
    player::Player,
};
pub struct Assets {
    pub player: AnimationsGroup,
    pub enemies: AnimationsGroup,
    pub tileset: Spritesheet,
    pub levels: Vec<World>,
    pub coin: Animation,

    pub start_btn: Animation,
    pub plus_btn: Animation,
    pub minus_btn: Animation,
    pub menu_body: Texture2D,

    pub coin_sfx: Sound,
    pub stomp_sfx: Sound,
    pub song: Sound,
}
impl Assets {
    pub async fn load() -> Self {
        Self {
            player: AnimationsGroup::from_file(include_bytes!("../assets/player.ase")),
            enemies: AnimationsGroup::from_file(include_bytes!("../assets/enemies.ase")),
            tileset: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/tileset.ase"), None),
                8.0,
            ),
            coin: Animation::from_file(include_bytes!("../assets/coin.ase")),

            start_btn: Animation::from_file(include_bytes!("../assets/start_btn.ase")),
            plus_btn: Animation::from_file(include_bytes!("../assets/plus_btn.ase")),
            minus_btn: Animation::from_file(include_bytes!("../assets/minus_btn.ase")),
            menu_body: load_ase_texture(include_bytes!("../assets/menu_body.ase"), None),

            coin_sfx: load_sound_from_bytes(include_bytes!("../assets/sfx/coin.wav"))
                .await
                .unwrap(),
            stomp_sfx: load_sound_from_bytes(include_bytes!("../assets/sfx/stomp.wav"))
                .await
                .unwrap(),
            song: load_sound_from_bytes(include_bytes!("../assets/sfx/song.wav"))
                .await
                .unwrap(),

            levels: vec![
                World::from_data(include_str!("../assets/menu.tmx")),
                World::from_data(include_str!("../assets/world.tmx")),
            ],
        }
    }
}

pub struct Animation {
    pub frames: Vec<(Texture2D, u32)>,
    pub total_length: u32,
}
impl Animation {
    pub fn from_file(bytes: &[u8]) -> Self {
        let ase = AsepriteFile::read(bytes).unwrap();
        let mut frames = Vec::new();
        let mut total_length = 0;
        for index in 0..ase.num_frames() {
            let frame = ase.frame(index);
            let img = frame.image();
            let new = Image {
                width: img.width() as u16,
                height: img.height() as u16,
                bytes: img.as_bytes().to_vec(),
            };
            let duration = frame.duration();
            total_length += duration;
            let texture = Texture2D::from_image(&new);
            texture.set_filter(FilterMode::Nearest);
            frames.push((texture, duration));
        }
        Self {
            frames,
            total_length,
        }
    }
    pub fn get_at_time(&self, mut time: u32) -> &Texture2D {
        time %= self.total_length;
        for (texture, length) in self.frames.iter() {
            if time >= *length {
                time -= length;
            } else {
                return texture;
            }
        }
        panic!()
    }
}

pub struct AnimationsGroup {
    #[expect(dead_code)]
    pub file: AsepriteFile,
    pub animations: Vec<Animation>,
    pub tag_names: HashMap<String, usize>,
}
impl AnimationsGroup {
    #[expect(dead_code)]
    pub fn get_by_name(&self, name: &str) -> &Animation {
        &self.animations[*self.tag_names.get(name).unwrap()]
    }
    pub fn from_file(bytes: &[u8]) -> Self {
        let ase = AsepriteFile::read(bytes).unwrap();
        let mut frames = Vec::new();
        for index in 0..ase.num_frames() {
            let frame = ase.frame(index);
            let img = frame.image();
            let new = Image {
                width: img.width() as u16,
                height: img.height() as u16,
                bytes: img.as_bytes().to_vec(),
            };
            let duration = frame.duration();
            let texture = Texture2D::from_image(&new);
            texture.set_filter(FilterMode::Nearest);
            frames.push((texture, duration));
        }
        let mut tag_frames = Vec::new();
        let mut offset = 0;

        let mut tag_names = HashMap::new();

        for i in 0..ase.num_tags() {
            let tag = ase.get_tag(i).unwrap();
            tag_names.insert(tag.name().to_string(), i as usize);
            let (start, end) = (tag.from_frame() as usize, tag.to_frame() as usize);
            let mut total_length = 0;
            let included_frames: Vec<(Texture2D, u32)> = frames
                .extract_if((start - offset)..(end - offset + 1), |_| true)
                .collect();
            for f in included_frames.iter() {
                total_length += f.1;
            }
            offset += end.abs_diff(start) + 1;
            tag_frames.push(Animation {
                frames: included_frames,
                total_length,
            });
        }
        Self {
            file: ase,
            animations: tag_frames,
            tag_names,
        }
    }
}
fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    #[expect(dead_code)]
    /// Same as `draw_tile`, except centered
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        self.draw_tile(
            screen_x - self.sprite_size / 2.0,
            screen_y - self.sprite_size / 2.0,
            tile_x,
            tile_y,
            params,
        );
    }
    /// Draws a single tile from the spritesheet
    pub fn draw_tile(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        let mut p = params.cloned().unwrap_or(DrawTextureParams::default());
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(&self.texture, screen_x, screen_y, WHITE, p);
    }
}

#[derive(Default, Clone)]
pub struct WorldState {
    pub enemies: Vec<Enemy>,
    pub broken_tiles: Vec<(i16, i16)>,
    pub coins: Vec<(i16, i16)>,
    pub taken_coins: usize,
}
pub struct World {
    pub collision: HashMap<(i16, i16), Chunk>,
    pub details: HashMap<(i16, i16), Chunk>,
    pub background: HashMap<(i16, i16), Chunk>,
    pub special: HashMap<(i16, i16), Chunk>,

    world_state: WorldState,
}
impl World {
    pub fn load_level(&self) -> (WorldState, Player) {
        (
            self.world_state.clone(),
            Player::new(self.get_player_spawn()),
        )
    }
    fn get_player_spawn(&self) -> Vec2 {
        let mut highest = i16::MAX;
        for chunk in self.collision.values().filter(|f| f.x == 0) {
            for row in 0..16 {
                let tile = chunk.tiles[row * 16];
                if tile != 0 {
                    let y = chunk.y + row as i16;
                    if y < highest {
                        highest = y;
                    }
                }
            }
        }
        vec2(1.0, (highest - 1) as f32 * 8.0)
    }
    #[expect(dead_code)]
    pub fn get_interactable_spawn(&self, tile_index: i16) -> Option<Vec2> {
        for chunk in self.special.values() {
            for (i, tile) in chunk.tiles.iter().enumerate() {
                if *tile == tile_index + 1 {
                    return Some(Vec2::new(
                        (i as i16 % 16 + chunk.x) as f32 * 8.0,
                        (i as i16 / 16 + chunk.y) as f32 * 8.0,
                    ));
                }
            }
        }
        None
    }
    pub fn from_data(xml: &str) -> Self {
        let collision = get_layer(xml, "collision");
        let detail = get_layer(xml, "detail");
        let special = get_layer(xml, "special");
        let background = get_layer(xml, "background");

        let mut world_state = WorldState::default();
        let special = get_all_chunks(special);
        for chunk in special.values() {
            for (index, tile) in chunk.tiles.iter().enumerate() {
                let x = (index % 16) as i16 + chunk.x;
                let y = (index / 16) as i16 + chunk.y;
                if *tile == 0 {
                    continue;
                } else if *tile == 1 {
                    world_state.coins.push((x, y));
                    continue;
                }

                let tile = *tile - 2;
                if tile >= 16 {
                    continue;
                }
                let Some(ty) = EnemyType::from_i16(tile) else {
                    warn!("ty {tile} doesnt exist!");
                    continue;
                };
                let enemy = Enemy::new(vec2(x as f32 * 8.0, y as f32 * 8.0), ty);
                world_state.enemies.push(enemy);
            }
        }

        World {
            collision: get_all_chunks(collision),
            details: get_all_chunks(detail),
            special,
            background: get_all_chunks(background),
            world_state,
        }
    }
}

pub struct Chunk {
    pub x: i16,
    pub y: i16,
    pub tiles: Vec<i16>,
}
impl Chunk {
    pub fn tile_at(&self, x: usize, y: usize) -> Option<i16> {
        if x > 16 || y > 16 {
            return None;
        }
        self.tiles.get(x + y * 16).cloned()
    }
}

fn get_all_chunks(xml: &str) -> HashMap<(i16, i16), Chunk> {
    let mut chunks = HashMap::new();
    let mut xml = xml.to_string();
    while let Some((current, remains)) = xml.split_once("</chunk>") {
        let new = parse_chunk(current);
        chunks.insert((new.x, new.y), new);
        xml = remains.to_string();
    }

    chunks
}

fn get_layer<'a>(xml: &'a str, layer: &str) -> &'a str {
    let split = format!(" name=\"{layer}");
    xml.split_once(&split)
        .unwrap()
        .1
        .split_once(">")
        .unwrap()
        .1
        .split_once("</layer>")
        .unwrap()
        .0
}

fn parse_chunk(xml: &str) -> Chunk {
    let (tag, data) = xml
        .split_once("<chunk ")
        .unwrap()
        .1
        .split_once(">")
        .unwrap();

    let x = tag
        .split_once("x=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();
    let y = tag
        .split_once("y=\"")
        .unwrap()
        .1
        .split_once("\"")
        .unwrap()
        .0
        .parse()
        .unwrap();

    let mut split = data.split(',');

    let mut chunk = vec![0; 16 * 16];
    for item in &mut chunk {
        let a = split.next().unwrap().trim();
        *item = a.parse().unwrap()
    }
    Chunk { x, y, tiles: chunk }
}
