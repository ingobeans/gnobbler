use std::borrow::Borrow;

use macroquad::prelude::*;

use crate::assets::*;

fn ceil_g(a: f32) -> f32 {
    if a < 0.0 { a.floor() } else { a.ceil() }
}

fn get_tile<T: Borrow<Chunk>>(chunks: &[T], x: i16, y: i16) -> i16 {
    if x < 0 {
        return 1;
    }
    let cx = ((x as f32 / 16.0).floor() * 16.0) as i16;
    let cy = ((y as f32 / 16.0).floor() * 16.0) as i16;
    let Some(chunk) = chunks.iter().find(|f| {
        let f: &Chunk = (*f).borrow();
        f.x == cx && f.y == cy
    }) else {
        return 0;
    };
    let chunk = chunk.borrow();
    let local_x = x - chunk.x;
    let local_y = y - chunk.y;
    chunk.tile_at(local_x as _, local_y as _).unwrap_or(0)
}

pub fn update_physicsbody(
    pos: Vec2,
    velocity: &mut Vec2,
    delta_time: f32,
    world: &World,
) -> (Vec2, bool) {
    let mut new = pos + *velocity * delta_time;

    let tile_x = pos.x / 8.0;
    let tile_y = pos.y / 8.0;

    let tiles_y = [
        (tile_x.trunc(), ceil_g(new.y / 8.0)),
        (ceil_g(tile_x), ceil_g(new.y / 8.0)),
        (tile_x.trunc(), (new.y / 8.0).trunc()),
        (ceil_g(tile_x), (new.y / 8.0).trunc()),
    ];

    let chunks_pos: [(i16, i16); 4] = std::array::from_fn(|f| {
        let cx = ((tiles_y[f].0 / 16.0).floor() * 16.0) as i16;
        let cy = ((tiles_y[f].1 / 16.0).floor() * 16.0) as i16;
        (cx, cy)
    });

    let chunks: Vec<&Chunk> = world
        .collision
        .iter()
        .filter(|f| chunks_pos.contains(&(f.x, f.y)))
        .collect();

    let mut grounded = false;
    for (tx, ty) in tiles_y {
        let tile = get_tile(&chunks, tx as i16, ty as i16);
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

    let chunks_pos: [(i16, i16); 4] = std::array::from_fn(|f| {
        let cx = ((tiles_x[f].0 / 16.0).floor() * 16.0) as i16;
        let cy = ((tiles_x[f].1 / 16.0).floor() * 16.0) as i16;
        (cx, cy)
    });

    let chunks: Vec<&Chunk> = world
        .collision
        .iter()
        .filter(|f| chunks_pos.contains(&(f.x, f.y)))
        .collect();

    for (tx, ty) in tiles_x {
        let tile = get_tile(&chunks, tx as i16, ty as i16);
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
    (new, grounded)
}
