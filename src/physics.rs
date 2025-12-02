use std::{borrow::Borrow, collections::HashMap};

use macroquad::prelude::*;

use crate::assets::*;

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

pub fn get_tile(chunks: &HashMap<(i16, i16), Chunk>, x: i16, y: i16) -> i16 {
    if x < 0 {
        return 1;
    }
    let tx = x / 16 * 16;
    let ty = y / 16 * 16;
    if let Some(c) = chunks.get(&(tx, ty)) {
        c.tile_at((x - tx) as usize, (y - ty) as usize).unwrap_or(0)
    } else {
        0
    }
}
pub fn update_physicsbody(
    pos: Vec2,
    velocity: &mut Vec2,
    delta_time: f32,
    world: &World,
) -> (Vec2, bool, bool) {
    let mut new = pos + *velocity * delta_time;
    let mut touched_death_tile = false;

    let tile_x = pos.x / 8.0;
    let tile_y = pos.y / 8.0;

    let tiles_y = [
        (tile_x.trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(tile_x), ceil_g(new.y / 8.0)),
        (tile_x.trunc(), (new.y / 8.0).trunc()),
        (ceil_g(tile_x), (new.y / 8.0).trunc()),
    ];

    let mut grounded = false;
    for (tx, ty) in tiles_y {
        let tile = get_tile(&world.collision, tx as i16, ty as i16);
        if tile != 0 || world.tile_entities.contains_key(&(tx as i16, ty as i16)) {
            let c = if velocity.y < 0.0 {
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
        if !touched_death_tile {
            let death_tile = get_tile(&world.death, tx as i16, ty as i16);
            touched_death_tile = death_tile != 0;
        }
        let tile = get_tile(&world.collision, tx as i16, ty as i16);
        if tile != 0 || world.tile_entities.contains_key(&(tx as i16, ty as i16)) {
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
    (new, grounded, touched_death_tile)
}
