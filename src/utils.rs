use impl_new_derive::ImplNew;
use macroquad::prelude::*;

pub const SCREEN_WIDTH: f32 = 256.0;
pub const SCREEN_HEIGHT: f32 = 144.0;

pub const GROUND_FRICTION: f32 = 0.17 * 60.0;
pub const AIR_DRAG: f32 = 0.15 * 60.0;
pub const GRAVITY: f32 = 0.17 * 3600.0;
pub const ACCELERATION: f32 = 2400.0 / 2.0;

pub const BOAT_WAIT_TIME: f32 = 0.2;
pub const BOAT_MOVE_SPEED: f32 = 64.0;

pub fn create_camera(w: f32, h: f32) -> Camera2D {
    let rt = render_target(w as u32, h as u32);
    rt.texture.set_filter(FilterMode::Nearest);

    Camera2D {
        render_target: Some(rt),
        zoom: Vec2::new(1.0 / w * 2.0, 1.0 / h * 2.0),
        ..Default::default()
    }
}
pub fn get_input_axis() -> Vec2 {
    let mut i = Vec2::ZERO;
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        i.x -= 1.0;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        i.x += 1.0;
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        i.y -= 1.0;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        i.y += 1.0;
    }
    i
}

#[derive(ImplNew)]
pub struct UIImageButton<'a> {
    pub pos: Vec2,
    pub texture: &'a Texture2D,
    pub hovered: &'a Texture2D,
    pub scale_factor: f32,
}
impl<'a> UIImageButton<'a> {
    pub fn is_hovered(&self) -> bool {
        let size = self.texture.size() * self.scale_factor;
        let mouse = mouse_position();
        (self.pos.x..self.pos.x + size.x).contains(&mouse.0)
            && (self.pos.y..self.pos.y + size.y).contains(&mouse.1)
    }
    pub fn draw(&self) {
        let texture = if self.is_hovered() {
            self.hovered
        } else {
            self.texture
        };
        draw_texture_ex(
            texture,
            self.pos.x.floor(),
            self.pos.y.floor(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.scale_factor * texture.size()),
                ..Default::default()
            },
        );
    }
}
