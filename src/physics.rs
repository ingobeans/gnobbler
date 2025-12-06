use std::collections::HashMap;

use macroquad::prelude::*;

use crate::assets::*;

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

pub fn get_tile(chunks: &HashMap<(i16, i16), Chunk>, x: i16, y: i16) -> i16 {
    if x < 0 {
        // make left level boundary act as wall
        return 50;
    }
    let tx = ((x as f32 / 16.0).floor() * 16.0) as i16;
    let ty = ((y as f32 / 16.0).floor() * 16.0) as i16;
    if let Some(c) = chunks.get(&(tx, ty)) {
        c.tile_at((x - tx) as usize, (y - ty) as usize).unwrap_or(0)
    } else {
        0
    }
}
#[derive(Clone, Copy)]
pub enum TileFlag {
    Collision,
    NoCollision,
    OneWayCollision,
    Death,
}
impl TileFlag {
    pub fn is_one_way(self) -> bool {
        matches!(self, TileFlag::OneWayCollision)
    }
    pub fn is_death(self) -> bool {
        matches!(self, TileFlag::Death)
    }
    pub fn is_collision(self) -> bool {
        matches!(self, TileFlag::Collision)
    }
    pub fn is_no_collision(self) -> bool {
        matches!(self, TileFlag::NoCollision)
    }
}
pub fn get_tile_flag(tile: i16) -> TileFlag {
    match tile - 1 {
        16..32 => TileFlag::Death,
        32..48 => TileFlag::OneWayCollision,
        48..64 => TileFlag::Collision,
        _ => TileFlag::NoCollision,
    }
}
pub fn update_physicsbody(
    pos: Vec2,
    velocity: &mut Vec2,
    delta_time: f32,
    world: &World,
    broken_tiles: &[(i16, i16)],
) -> (Vec2, bool, bool, Option<(i16, i16)>) {
    let mut new = pos + *velocity * delta_time;
    let mut touched_death_tile = false;
    let mut broke_block = None;
    let original_velocity = *velocity;

    let tile_x = pos.x / 8.0;
    let tile_y = pos.y / 8.0;

    let tiles_y = [
        (tile_x.trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(tile_x), ceil_g(new.y / 8.0)),
        (tile_x.trunc(), (new.y / 8.0).trunc()),
        (ceil_g(tile_x), (new.y / 8.0).trunc()),
    ];

    let mut grounded = false;
    for (i, (tx, ty)) in tiles_y.into_iter().enumerate() {
        let tile = get_tile(&world.collision, tx as i16, ty as i16);
        let flag = get_tile_flag(tile);
        if (flag.is_collision() && !(tile == 49 && broken_tiles.contains(&(tx as i16, ty as i16))))
            || (i < 2 && velocity.y > 0.0 && flag.is_one_way())
        {
            let c = if velocity.y < 0.0 {
                if tile == 49 && broke_block.is_none() {
                    broke_block = Some((tx as i16, ty as i16));
                }
                tile_y.floor() * 8.0
            } else {
                grounded = true;
                tile_y.ceil() * 8.0
            };
            new.y = c;
            velocity.y = 0.0;
            break;
        }
    }
    let tiles_x = [
        ((new.x / 8.0).trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(new.x / 8.0), ceil_g(new.y / 8.0)),
        (ceil_g(new.x / 8.0), (new.y / 8.0).trunc()),
        ((new.x / 8.0).trunc(), (new.y / 8.0).trunc()),
    ];

    for (tx, ty) in tiles_x {
        let tile = get_tile(&world.collision, tx as i16, ty as i16);
        let flag = get_tile_flag(tile);
        if !touched_death_tile
            && tx > 0.0
            && (new + vec2(4.0, 0.0)).distance_squared(vec2(tx + 0.5, ty) * 8.0) < 16.0
        {
            touched_death_tile = flag.is_death();
        }
        if tile == 113
            && original_velocity.y > 0.0
            && (new + vec2(4.0, 0.0)).distance_squared(vec2(tx + 0.5, ty) * 8.0) < 16.0
        {
            velocity.y = -4.5 * 60.0;
        }
        if flag.is_collision() && !(tile == 49 && broken_tiles.contains(&(tx as i16, ty as i16))) {
            let c = if velocity.x < 0.0 {
                tile_x.floor() * 8.0
            } else {
                tile_x.ceil() * 8.0
            };
            new.x = c;
            velocity.x = 0.0;
            break;
        }
    }
    (new, grounded, touched_death_tile, broke_block)
}
